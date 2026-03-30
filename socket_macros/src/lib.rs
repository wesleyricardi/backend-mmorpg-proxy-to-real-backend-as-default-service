use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use syn::{
  parse_macro_input, Attribute, Expr, ExprLit, Fields, Ident, ItemStruct, Lit, MetaNameValue,
  Result, Token, Type,
};

/// Attribute macro for socket protocol structs.
///
/// Generates:
/// - internal C representation (`#[repr(C)]`)
/// - Rust <-> C conversion
/// - `socket::SocketPacket` (`to_bytes` / `from_bytes`)
/// - `socket::FromBody`
///
/// Field options (on struct fields):
/// - `#[socket(string_size = N)]` for `String` <-> `char[N]`
/// - `#[socket(vec_size = N)]` for `Vec<T>` <-> `T[N]`
///
/// Route options (on struct, optional):
/// - `#[socket(RouteName = 0x1234)]`
/// - Can declare more than one route in the same attribute.
///
/// Example:
/// ```rust
/// use socket::socket;
///
/// #[socket]
/// #[derive(Debug, Clone)]
/// pub struct ChatMessage {
///   pub ip: u32,
///   #[socket(string_size = 256)]
///   pub message: String,
///   #[socket(vec_size = 8)]
///   pub tags: Vec<u16>,
/// }
/// ```
#[proc_macro_attribute]
pub fn socket(attr: TokenStream, item: TokenStream) -> TokenStream {
  let routes = parse_macro_input!(attr with Punctuated::<MetaNameValue, Token![,]>::parse_terminated);
  let mut item_struct = parse_macro_input!(item as ItemStruct);

  let struct_ident = item_struct.ident.clone();
  let c_mod_ident = format_ident!("__socket_c_{}", struct_ident.to_string().to_lowercase());
  let c_ident = format_ident!("{}C", struct_ident);

  let fields_named = match &mut item_struct.fields {
    Fields::Named(fields) => &mut fields.named,
    _ => {
      return syn::Error::new_spanned(&item_struct, "socket only supports named-field structs")
        .to_compile_error()
        .into();
    }
  };

  let mut c_fields = Vec::new();
  let mut to_c_assignments = Vec::new();
  let mut from_c_assignments = Vec::new();

  for field in fields_named.iter_mut() {
    let field_ident = field.ident.clone().expect("named field expected");
    let meta = match extract_field_socket_meta(&mut field.attrs) {
      Ok(v) => v,
      Err(err) => return err.to_compile_error().into(),
    };

    let c_type = match c_type_for(&field.ty, meta.string_size, meta.vec_size) {
      Ok(v) => v,
      Err(err) => return err.to_compile_error().into(),
    };

    let to_expr = match to_c_expr(
      &field.ty,
      meta.string_size,
      meta.vec_size,
      quote! { self.#field_ident },
    ) {
      Ok(v) => v,
      Err(err) => return err.to_compile_error().into(),
    };

    let from_expr = match from_c_expr(
      &field.ty,
      meta.string_size,
      meta.vec_size,
      quote! { value.#field_ident },
    ) {
      Ok(v) => v,
      Err(err) => return err.to_compile_error().into(),
    };

    c_fields.push(quote! { pub #field_ident: #c_type });
    to_c_assignments.push(quote! { #field_ident: #to_expr });
    from_c_assignments.push(quote! { #field_ident: #from_expr });
  }

  let route_names = routes.iter().map(|route| &route.path);
  let route_codes = routes.iter().map(|route| &route.value);

  let expanded = quote! {
    #item_struct

    mod #c_mod_ident {
      use super::*;

      #[repr(C)]
      #[derive(Clone, Copy, Debug)]
      pub struct #c_ident {
        #(#c_fields,)*
      }
    }

    impl socket::SocketPacket for #struct_ident {
      type CRepr = #c_mod_ident::#c_ident;

      fn to_c_repr(&self) -> Self::CRepr {
        #c_mod_ident::#c_ident {
          #(#to_c_assignments,)*
        }
      }

      fn from_c_repr(value: Self::CRepr) -> Self {
        Self {
          #(#from_c_assignments,)*
        }
      }
    }

    impl #struct_ident {
      pub fn to_bytes(&self) -> Vec<u8> {
        <Self as socket::SocketPacket>::to_bytes(self)
      }

      pub fn from_bytes(bytes: &[u8]) -> Result<Self, socket::SocketPacketError> {
        <Self as socket::SocketPacket>::from_bytes(bytes)
      }
    }

    impl socket::FromBody for #struct_ident {
      fn transmute(
        body: std::sync::Arc<[u8]>,
        _size: usize,
        _code: i32,
      ) -> Result<Self, socket::SocketPacketError> {
        <Self as socket::SocketPacket>::from_bytes(&body)
      }
    }

    impl socket::SocketRouteMeta for #struct_ident {
      fn routes() -> &'static [socket::RouteDef] {
        &[
          #(socket::RouteDef { name: stringify!(#route_names), code: (#route_codes) as i32 },)*
        ]
      }
    }
  };

  expanded.into()
}

struct RouteEntry {
  route: Ident,
  _arrow: Token![=>],
  ty: Type,
  _eq: Token![=],
  code: Expr,
}

impl Parse for RouteEntry {
  fn parse(input: ParseStream) -> Result<Self> {
    Ok(Self {
      route: input.parse()?,
      _arrow: input.parse()?,
      ty: input.parse()?,
      _eq: input.parse()?,
      code: input.parse()?,
    })
  }
}

/// Aggregates route declarations for one protocol module.
///
/// Generates:
/// - `pub mod route_code` with route constants
/// - `pub enum Message`
/// - `socket::IntoMessageBuffer` for `Message`
///
/// Example:
/// ```rust
/// socket::protocol_module! {
///   Ping => Ping = 0x49000000,
///   WhisperMessage => ChatMessage = 0x48471005,
/// }
/// ```
#[proc_macro]
pub fn protocol_module(input: TokenStream) -> TokenStream {
  let entries = parse_macro_input!(input with Punctuated::<RouteEntry, Token![,]>::parse_terminated);

  let route_names = entries.iter().map(|entry| &entry.route);
  let route_codes = entries.iter().map(|entry| &entry.code);
  let route_variants = entries.iter().map(|entry| &entry.route);
  let route_types = entries.iter().map(|entry| &entry.ty);
  let route_match_variants = entries.iter().map(|entry| &entry.route);
  let route_match_codes = entries.iter().map(|entry| &entry.route);

  let expanded = quote! {
    pub mod route_code {
      #(
        #[allow(non_snake_case, non_upper_case_globals)]
        pub const #route_names: i32 = (#route_codes) as i32;
      )*
    }

    #[derive(Debug)]
    pub enum Message {
      #(#route_variants(#route_types),)*
    }

    impl socket::IntoMessageBuffer for Message {
      fn into_message_buffer(self) -> socket::Message<socket::Bytes> {
        match self {
          #(Message::#route_match_variants(v) => socket::Message::new(route_code::#route_match_codes, socket::SocketPacket::to_bytes(&v)),)*
        }
      }
    }
  };

  expanded.into()
}

#[derive(Default)]
struct FieldSocketMeta {
  string_size: Option<usize>,
  vec_size: Option<usize>,
}

fn extract_field_socket_meta(attrs: &mut Vec<Attribute>) -> Result<FieldSocketMeta> {
  let mut meta = FieldSocketMeta::default();
  let mut keep = Vec::new();

  for attr in attrs.drain(..) {
    if !attr.path().is_ident("socket") {
      keep.push(attr);
      continue;
    }

    let parsed = attr.parse_args_with(Punctuated::<MetaNameValue, Token![,]>::parse_terminated)?;
    for item in parsed {
      let value = match item.value {
        Expr::Lit(ExprLit {
          lit: Lit::Int(ref lit),
          ..
        }) => lit.base10_parse::<usize>()?,
        _ => {
          return Err(syn::Error::new_spanned(
            item.value,
            "socket field attribute value must be an integer",
          ));
        }
      };

      if item.path.is_ident("string_size") {
        meta.string_size = Some(value);
      } else if item.path.is_ident("vec_size") {
        meta.vec_size = Some(value);
      } else {
        return Err(syn::Error::new_spanned(item, "unknown socket field attribute"));
      }
    }
  }

  *attrs = keep;
  Ok(meta)
}

fn c_type_for(
  ty: &Type,
  string_size: Option<usize>,
  vec_size: Option<usize>,
) -> Result<proc_macro2::TokenStream> {
  if let Some(len) = wire_string_len(ty) {
    return Ok(quote! { [u8; #len] });
  }

  if is_string_type(ty) {
    let Some(len) = string_size else {
      return Err(syn::Error::new_spanned(
        ty,
        "String fields must declare #[socket(string_size = N)]",
      ));
    };
    return Ok(quote! { [u8; #len] });
  }

  if let Some(inner) = vec_inner_type(ty) {
    let Some(len) = vec_size else {
      return Err(syn::Error::new_spanned(
        ty,
        "Vec fields must declare #[socket(vec_size = N)]",
      ));
    };
    let inner_c = c_type_for(inner, None, None)?;
    return Ok(quote! { [#inner_c; #len] });
  }

  if let Type::Array(array) = ty {
    let inner = c_type_for(&array.elem, None, None)?;
    let len = &array.len;
    return Ok(quote! { [#inner; #len] });
  }

  if is_primitive_type(ty) {
    return Ok(quote! { #ty });
  }

  Ok(quote! { <#ty as socket::SocketPacket>::CRepr })
}

fn to_c_expr(
  ty: &Type,
  string_size: Option<usize>,
  vec_size: Option<usize>,
  value_expr: proc_macro2::TokenStream,
) -> Result<proc_macro2::TokenStream> {
  if wire_string_len(ty).is_some() {
    return Ok(quote! { #value_expr.clone().into_raw() });
  }

  if is_string_type(ty) {
    let Some(len) = string_size else {
      return Err(syn::Error::new_spanned(
        ty,
        "String fields must declare #[socket(string_size = N)]",
      ));
    };
    return Ok(quote! { socket::string_to_fixed::<#len>(&#value_expr) });
  }

  if let Some(inner) = vec_inner_type(ty) {
    let Some(_len) = vec_size else {
      return Err(syn::Error::new_spanned(
        ty,
        "Vec fields must declare #[socket(vec_size = N)]",
      ));
    };
    let len = vec_size.expect("vec_size already validated");
    let inner_expr = to_c_expr(inner, None, None, quote! { value })?;
    return Ok(quote! {
      {
        let mut out = [unsafe { std::mem::zeroed() }; #len];
        for (i, value) in #value_expr.iter().cloned().take(#len).enumerate() {
          out[i] = { #inner_expr };
        }
        out
      }
    });
  }

  if let Type::Array(array) = ty {
    let inner_expr = to_c_expr(&array.elem, None, None, quote! { value })?;
    return Ok(quote! {
      std::array::from_fn(|i| {
        let value = #value_expr[i].clone();
        #inner_expr
      })
    });
  }

  if is_primitive_type(ty) {
    return Ok(quote! { #value_expr });
  }

  Ok(quote! { <#ty as socket::SocketPacket>::to_c_repr(&#value_expr) })
}

fn from_c_expr(
  ty: &Type,
  string_size: Option<usize>,
  vec_size: Option<usize>,
  value_expr: proc_macro2::TokenStream,
) -> Result<proc_macro2::TokenStream> {
  if wire_string_len(ty).is_some() {
    return Ok(quote! { <#ty>::from_raw(#value_expr) });
  }

  if is_string_type(ty) {
    if string_size.is_none() {
      return Err(syn::Error::new_spanned(
        ty,
        "String fields must declare #[socket(string_size = N)]",
      ));
    }
    return Ok(quote! { socket::fixed_to_string(&#value_expr) });
  }

  if let Some(inner) = vec_inner_type(ty) {
    let _ = vec_size;
    let inner_expr = from_c_expr(inner, None, None, quote! { value })?;
    return Ok(quote! {
      #value_expr
        .into_iter()
        .map(|value| { #inner_expr })
        .collect::<Vec<_>>()
    });
  }

  if let Type::Array(array) = ty {
    let inner_expr = from_c_expr(&array.elem, None, None, quote! { value })?;
    return Ok(quote! {
      std::array::from_fn(|i| {
        let value = #value_expr[i];
        #inner_expr
      })
    });
  }

  if is_primitive_type(ty) {
    return Ok(quote! { #value_expr });
  }

  Ok(quote! { <#ty as socket::SocketPacket>::from_c_repr(#value_expr) })
}

fn is_string_type(ty: &Type) -> bool {
  match ty {
    Type::Path(path) => path.path.is_ident("String"),
    _ => false,
  }
}

fn wire_string_len(ty: &Type) -> Option<proc_macro2::TokenStream> {
  let Type::Path(path) = ty else {
    return None;
  };

  let segment = path.path.segments.last()?;
  if segment.ident != "WireString" {
    return None;
  }

  let syn::PathArguments::AngleBracketed(args) = &segment.arguments else {
    return None;
  };

  let first = args.args.first()?;
  match first {
    syn::GenericArgument::Const(expr) => Some(quote! { #expr }),
    _ => None,
  }
}

fn is_primitive_type(ty: &Type) -> bool {
  let Type::Path(path) = ty else {
    return false;
  };

  let Some(ident) = path.path.get_ident() else {
    return false;
  };

  matches!(
    ident.to_string().as_str(),
    "u8"
      | "u16"
      | "u32"
      | "u64"
      | "u128"
      | "usize"
      | "i8"
      | "i16"
      | "i32"
      | "i64"
      | "i128"
      | "isize"
      | "f32"
      | "f64"
      | "bool"
  )
}

fn vec_inner_type(ty: &Type) -> Option<&Type> {
  let Type::Path(path) = ty else {
    return None;
  };
  let segment = path.path.segments.last()?;
  if segment.ident != "Vec" {
    return None;
  }

  let syn::PathArguments::AngleBracketed(args) = &segment.arguments else {
    return None;
  };
  let first = args.args.first()?;
  let syn::GenericArgument::Type(inner_ty) = first else {
    return None;
  };
  Some(inner_ty)
}
