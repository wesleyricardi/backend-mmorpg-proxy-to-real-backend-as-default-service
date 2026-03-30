use std::{collections::HashSet, env, fs, path::PathBuf};

#[derive(Clone, Copy)]
struct Spec {
    /// Nome do ItemType, ex: "WA1"
    ty: &'static str,
    /// Range de variante (1..=98)
    start: u8,
    end: u8,
}

fn main() {
    // >>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>
    // EDITE AQUI: seus ranges por tipo (ty => start..=end)
    // >>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>
    let specs: &[Spec] = &[
        Spec {
            ty: "WA1",
            start: 1,
            end: 98,
        },
        Spec {
            ty: "WC1",
            start: 1,
            end: 98,
        },
        Spec {
            ty: "WH1",
            start: 1,
            end: 98,
        },
        Spec {
            ty: "WM1",
            start: 1,
            end: 98,
        },
        Spec {
            ty: "WP1",
            start: 1,
            end: 98,
        },
        Spec {
            ty: "WS1",
            start: 1,
            end: 98,
        },
        Spec {
            ty: "WS2",
            start: 1,
            end: 98,
        },
        Spec {
            ty: "WT1",
            start: 1,
            end: 98,
        },
        Spec {
            ty: "WN1",
            start: 1,
            end: 98,
        },
        Spec {
            ty: "WD1",
            start: 1,
            end: 98,
        },
        Spec {
            ty: "WV1",
            start: 1,
            end: 98,
        },
        Spec {
            ty: "DA1",
            start: 1,
            end: 98,
        },
        Spec {
            ty: "DB1",
            start: 1,
            end: 98,
        },
        Spec {
            ty: "DG1",
            start: 1,
            end: 98,
        },
        Spec {
            ty: "DS1",
            start: 1,
            end: 98,
        },
        Spec {
            ty: "DA2",
            start: 1,
            end: 98,
        },
        Spec {
            ty: "CA1",
            start: 1,
            end: 98,
        },
        Spec {
            ty: "CA2",
            start: 1,
            end: 98,
        },
        Spec {
            ty: "CA5",
            start: 1,
            end: 98,
        },
        Spec {
            ty: "CA6",
            start: 1,
            end: 98,
        },
        Spec {
            ty: "DA3",
            start: 1,
            end: 98,
        },
        Spec {
            ty: "DA4",
            start: 1,
            end: 98,
        },
        Spec {
            ty: "OA1",
            start: 1,
            end: 98,
        },
        Spec {
            ty: "OA2",
            start: 1,
            end: 98,
        },
        Spec {
            ty: "OM1",
            start: 1,
            end: 98,
        },
        Spec {
            ty: "OR1",
            start: 1,
            end: 98,
        },
        Spec {
            ty: "OR2",
            start: 1,
            end: 98,
        },
        Spec {
            ty: "OS1",
            start: 1,
            end: 98,
        },
        Spec {
            ty: "FO1",
            start: 1,
            end: 98,
        },
        Spec {
            ty: "SE1",
            start: 1,
            end: 98,
        },
        Spec {
            ty: "PR1",
            start: 1,
            end: 98,
        },
        Spec {
            ty: "PR2",
            start: 1,
            end: 98,
        },
        Spec {
            ty: "PR3",
            start: 1,
            end: 98,
        },
        Spec {
            ty: "PR4",
            start: 1,
            end: 98,
        },
        Spec {
            ty: "OE1",
            start: 1,
            end: 98,
        },
        Spec {
            ty: "PM1",
            start: 1,
            end: 98,
        },
        Spec {
            ty: "PL1",
            start: 1,
            end: 98,
        },
        Spec {
            ty: "PS1",
            start: 1,
            end: 98,
        },
        Spec {
            ty: "GG1",
            start: 1,
            end: 98,
        },
        Spec {
            ty: "BS1",
            start: 1,
            end: 98,
        },
        Spec {
            ty: "EC1",
            start: 1,
            end: 98,
        },
        Spec {
            ty: "QT1",
            start: 1,
            end: 98,
        },
        Spec {
            ty: "SP1",
            start: 1,
            end: 98,
        },
        Spec {
            ty: "GP1",
            start: 1,
            end: 98,
        },
        Spec {
            ty: "QW1",
            start: 1,
            end: 98,
        },
        Spec {
            ty: "GF1",
            start: 1,
            end: 98,
        },
        Spec {
            ty: "PZ1",
            start: 1,
            end: 98,
        },
        Spec {
            ty: "PZ2",
            start: 1,
            end: 98,
        },
        Spec {
            ty: "CH1",
            start: 1,
            end: 98,
        },
        Spec {
            ty: "SD2",
            start: 1,
            end: 98,
        },
        Spec {
            ty: "BC1",
            start: 1,
            end: 98,
        },
        Spec {
            ty: "BI1",
            start: 1,
            end: 98,
        },
        Spec {
            ty: "BI2",
            start: 1,
            end: 98,
        },
        Spec {
            ty: "GP2",
            start: 1,
            end: 98,
        },
        Spec {
            ty: "MA1",
            start: 1,
            end: 98,
        },
        Spec {
            ty: "MA2",
            start: 1,
            end: 98,
        },
        Spec {
            ty: "BI3",
            start: 1,
            end: 98,
        },
        Spec {
            ty: "EV1",
            start: 1,
            end: 98,
        },
        Spec {
            ty: "EV2",
            start: 1,
            end: 98,
        },
        Spec {
            ty: "WR1",
            start: 1,
            end: 98,
        },
        Spec {
            ty: "DR1",
            start: 1,
            end: 98,
        },
        Spec {
            ty: "RR1",
            start: 1,
            end: 98,
        },
    ];

    // validações no build
    // - start/end coerentes
    // - sem specs duplicadas pro mesmo tipo (pra evitar gerar const repetida)
    let mut seen = HashSet::new();
    for s in specs {
        assert!(
            s.start >= 1,
            "Range inválido para {}: start precisa ser >= 1",
            s.ty
        );
        assert!(
            s.start <= s.end,
            "Range inválido para {}: {}..={}. start precisa ser <= end.",
            s.ty,
            s.start,
            s.end
        );
        assert!(
            seen.insert(s.ty),
            "Spec duplicada para o tipo {}. Deixe apenas uma entrada por tipo.",
            s.ty
        );
    }

    // destino fixo: src/domain/item/item_code.rs
    let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    let dest = manifest_dir
        .join("src")
        .join("domain")
        .join("item")
        .join("item_code.rs");

    let code = generate_item_code_rs(specs);

    // só escreve se mudou
    let needs_write = match fs::read_to_string(&dest) {
        Ok(existing) => existing != code,
        Err(_) => true,
    };
    if needs_write {
        fs::write(&dest, code).unwrap();
    }

    println!("cargo:rerun-if-changed=build.rs");
}

fn generate_item_code_rs(specs: &[Spec]) -> String {
    let mut out = String::new();

    out.push_str("// @generated by build.rs — NÃO EDITE À MÃO\n");
    out.push_str("// Este arquivo é reescrito durante o build.\n\n");

    // Ajuste este `use` conforme a sua árvore de módulos:
    out.push_str("use crate::domain::item::dto::ItemType;\n\n");

    out.push_str("#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]\n");
    out.push_str("pub struct ItemCode(u32);\n\n");

    out.push_str("#[derive(Debug, Clone, Copy, PartialEq, Eq)]\n");
    out.push_str("pub struct InvalidItemCode(pub u32);\n\n");

    out.push_str("impl ItemCode {\n");
    out.push_str("    pub const TYPE_MASK: u32 = 0xFFFF_0000;\n");
    out.push_str("    pub const VAR_MASK:  u32 = 0x0000_FF00;\n\n");
    out.push_str("    pub const fn raw(self) -> u32 { self.0 }\n");
    out.push_str("    pub const fn ty_bits(self) -> u32 { self.0 & Self::TYPE_MASK }\n");
    out.push_str(
        "    pub const fn variant(self) -> u8 { ((self.0 & Self::VAR_MASK) >> 8) as u8 }\n\n",
    );

    // ---------- NOVO: valida existência por (ty, variant) conforme specs ----------
    out.push_str("    pub const fn is_defined(ty: u32, variant: u8) -> bool {\n");
    out.push_str("        if variant < 1 { return false; }\n");
    out.push_str("        ");

    for (i, s) in specs.iter().enumerate() {
        out.push_str("(ty == (ItemType::");
        out.push_str(s.ty);
        out.push_str(" as u32) && (variant >= ");
        out.push_str(&s.start.to_string());
        out.push_str(" && variant <= ");
        out.push_str(&s.end.to_string());
        out.push_str("))");

        if i + 1 != specs.len() {
            out.push_str(" ||\n        ");
        } else {
            out.push_str("\n");
        }
    }

    out.push_str("    }\n\n");

    // ---------- Consts geradas: ItemCode::WA109, ItemCode::WA110, etc. ----------
    for s in specs {
        for v in s.start..=s.end {
            let const_name = if v < 10 {
                // WA1 + 09 => WA109
                format!("{}0{}", s.ty, v)
            } else {
                // WA1 + 10 => WA110
                format!("{}{}", s.ty, v)
            };

            out.push_str("    pub const ");
            out.push_str(&const_name);
            out.push_str(": ItemCode = ItemCode((");
            out.push_str("ItemType::");
            out.push_str(s.ty);
            out.push_str(" as u32) | ((");
            out.push_str(&v.to_string());
            out.push_str("u32) << 8));\n");
        }
        out.push('\n');
    }

    out.push_str("}\n\n");

    // ---------- TryFrom<u32>: aceita SOMENTE se is_defined(ty, variant) ----------
    out.push_str("impl TryFrom<u32> for ItemCode {\n");
    out.push_str("    type Error = InvalidItemCode;\n\n");
    out.push_str("    fn try_from(value: u32) -> Result<Self, Self::Error> {\n");
    out.push_str("        let ty = value & ItemCode::TYPE_MASK;\n");
    out.push_str("        let variant = ((value & ItemCode::VAR_MASK) >> 8) as u8;\n");
    out.push_str("        if !ItemCode::is_defined(ty, variant) {\n");
    out.push_str("            return Err(InvalidItemCode(value));\n");
    out.push_str("        }\n");
    out.push_str("        Ok(ItemCode(value))\n");
    out.push_str("    }\n");
    out.push_str("}\n");

    out
}
