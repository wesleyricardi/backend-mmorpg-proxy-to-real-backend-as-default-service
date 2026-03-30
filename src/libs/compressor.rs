pub fn decode(src_data: &[u8], d_count: usize) -> Vec<u8> {
    if src_data.len() < 4 {
        return Vec::new();
    }

    let size = u32::from_le_bytes([src_data[0], src_data[1], src_data[2], src_data[3]]) as usize;

    let mut counter = 4;
    let mut dec_counter = 0;
    let mut dest_data = vec![0u8; d_count];

    while counter < size && counter < src_data.len() {
        if src_data[counter] & 0x80 != 0 {
            let len = src_data[counter] & 0x7F;
            counter += 1;
            for _ in 0..len {
                if dec_counter >= d_count {
                    dest_data.truncate(dec_counter);
                    return dest_data;
                }
                dest_data[dec_counter] = 0;
                dec_counter += 1;
            }
        } else {
            let len = src_data[counter] & 0x7F;
            counter += 1;
            for _ in 0..len {
                if dec_counter >= d_count || counter >= src_data.len() {
                    dest_data.truncate(dec_counter);
                    return dest_data;
                }
                dest_data[dec_counter] = src_data[counter];
                counter += 1;
                dec_counter += 1;
            }
        }
    }

    dest_data.truncate(dec_counter);
    dest_data
}

pub fn encode(src_data: &[u8], size: usize, dest_size: usize) -> Vec<u8> {
    let mut counter = 0;
    let mut enc_counter = 4;
    let lsize = size - 1;
    let mut dest_data = vec![0; dest_size];

    while counter < size {
        if counter >= size || enc_counter >= dest_size {
            break;
        }

        let c1 = src_data[counter];
        let c2 = if counter < lsize {
            src_data[counter + 1]
        } else {
            c1
        };

        if c1 == 0 && c2 == 0 {
            let mut cnt = 0;
            for _ in 0..0x7F {
                if counter >= size {
                    break;
                }
                if src_data[counter] != 0 {
                    break;
                }
                counter += 1;
                cnt += 1;
            }
            if enc_counter >= dest_size {
                break;
            }
            dest_data[enc_counter] = cnt | 0x80;
            enc_counter += 1;
        } else {
            let p_set = enc_counter;
            enc_counter += 1;
            let mut cnt = 0;

            for _ in 0..0x7F {
                let c1 = src_data[counter];
                let c2 = if counter < lsize {
                    src_data[counter + 1]
                } else {
                    c1
                };

                if counter >= size || enc_counter >= dest_size || (c1 == 0 && c2 == 0) {
                    break;
                }
                dest_data[enc_counter] = src_data[counter];
                enc_counter += 1;
                counter += 1;
                cnt += 1;
            }
            dest_data[p_set] = cnt;
        }
    }

    dest_data[0..4].copy_from_slice(&(enc_counter as u32).to_le_bytes());

    dest_data.truncate(enc_counter);
    dest_data
}
