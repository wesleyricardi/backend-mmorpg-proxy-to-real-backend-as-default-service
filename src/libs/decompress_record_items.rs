use crate::{
    error::{AppError, ParseErrorKind},
    protocol,
};

pub fn decode_compress(lp_src_data: &[u8]) -> Vec<u8> {
    crate::libs::compressor::decode(lp_src_data, 0x7FFFF)
}

pub fn encode_compress(lp_src_data: &[u8], size: usize) -> Vec<u8> {
    crate::libs::compressor::encode(lp_src_data, size, 0x7FFFF)
}

pub fn get_record_items(
    compress: &[u8],
    item_count: usize,
) -> Result<Vec<protocol::item::RecordItem>, AppError> {
    get_record_item_chunks(compress, item_count)?
        .into_iter()
        .map(|record_item_decompress| {
            protocol::item::RecordItem::from_bytes(&record_item_decompress).map_err(|err| {
                log::error!(
                    "Failed to parse record item from decompressed data: {:?}",
                    err
                );
                crate::parse_error!(ParseErrorKind::RecordItem, "failed to parse record item")
            })
        })
        .collect()
}

pub fn get_record_item_chunks(
    compress: &[u8],
    item_count: usize,
) -> Result<Vec<Vec<u8>>, AppError> {
    let mut record_item_chunks: Vec<Vec<u8>> = Vec::new();

    let mut index: usize = 0;
    for _ in 0..item_count {
        if index + 4 > compress.len() {
            return Err(crate::parse_error!(
                ParseErrorKind::RecordItem,
                "record item header exceeds compressed buffer: index={}, len={}",
                index,
                compress.len()
            ));
        }

        let current_item_size: usize = u32::from_le_bytes([
            compress[index],
            compress[index + 1],
            compress[index + 2],
            compress[index + 3],
        ]) as usize;

        let end_item = index + current_item_size;

        if current_item_size == 0 || end_item > compress.len() {
            return Err(crate::parse_error!(
                ParseErrorKind::RecordItem,
                "invalid record item compressed chunk size: size={}, index={}, len={}",
                current_item_size,
                index,
                compress.len()
            ));
        }

        let record_item_decompress = decode_compress(&compress[index..end_item]);
        record_item_chunks.push(record_item_decompress);

        index += current_item_size;
    }

    Ok(record_item_chunks)
}
