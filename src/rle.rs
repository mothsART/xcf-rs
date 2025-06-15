pub fn short_run_len_identical(verbatim: &[u8]) -> Vec<u8> {
    if verbatim.len() < 1 {
        panic!("Wrong verbatim lengh : {}", verbatim.len());
    }
    vec![(verbatim.len() - 1) as u8, verbatim[0]]
}

pub fn long_run_identical(verbatim: &[u8]) -> Vec<u8> {
    if verbatim.len() < 127 {
        panic!("Wrong verbatim lengh : {}", verbatim.len());
    }
    // verbatim_len = p*256+q
    let p = verbatim.len() / 256;
    let q = verbatim.len() % 256;
    vec![127, p as u8, q as u8, verbatim[0]]
}

pub fn run_identical(verbatim: &[u8]) -> Vec<u8> {
    if verbatim.len() > 126 {
        return long_run_identical(&verbatim);
    }
    short_run_len_identical(&verbatim)
}

pub fn short_run_len_diff(verbatim: &[u8]) -> Vec<u8> {
    if verbatim.len() < 1 {
        panic!("Wrong verbatim lengh : {}", verbatim.len());
    }
    let mut r = vec![(256 - verbatim.len()) as u8];
    r.extend_from_slice(verbatim);
    r
}

pub fn long_run_diff(verbatim: &[u8]) -> Vec<u8> {
    if verbatim.len() < 127 {
        panic!("Wrong verbatim lengh : {}", verbatim.len());
    }
    // verbatim_len = p*256+q
    let p = verbatim.len() / 256;
    let q = verbatim.len() % 256;
    let mut r = vec![128, p as u8, q as u8];
    r.extend_from_slice(verbatim);
    r
}

pub fn run_diff(verbatim: &[u8]) -> Vec<u8> {
    if verbatim.len() > 126 {
        return long_run_diff(&verbatim);
    }
    short_run_len_diff(&verbatim)
}

pub fn is_identiqual(values: &[u8]) -> bool {
    if values.len() < 2 {
        panic!("Wrong values lengh : {}", values.len());
    }
    let value = values[0];
    for v in &values[1..values.len()] {
        if value != *v {
            return false;
        }
    }
    return true;
}

// https://testing.developer.gimp.org/core/standards/xcf/#rle-compressed-tile-data
pub fn rle_compress(data: &Vec<u8>) -> Vec<u8> {
    let mut compress_data = vec![];
    let mut verbatim: Vec<u8> = vec![];
    let mut i = 0;
    for byte in data {
        i += 1;
        verbatim.push(*byte);

        if i >= data.len() - 1 {
            break;
        }
        let val = *byte;
        let mut val_last_1 = val;
        let mut val_last_2 = val;
        if i > 1 {
            val_last_1 = data[i - 2];
        }
        if i > 2 {
            val_last_2 = data[i - 3];
        }
        let val_1 = data[i];
        let val_2 = data[i + 1];
        let mut val_3 = val_2;
        if i < data.len() - 2 {
            val_3 = data[i + 2];
        }

        if i == 2
        && val_last_1 == val
        && val != val_1 {
            let buffer = short_run_len_identical(&verbatim);
            compress_data.extend_from_slice(&buffer);
            verbatim = vec![];
            continue;
        }
        if i == 1
        && val != val_1
        && val_1 == val_2
        && val_2 == val_3 {
            let buffer = run_diff(&verbatim);
            compress_data.extend_from_slice(&buffer);
            verbatim = vec![];
            continue;
        }
        if i > 2
        && val_last_2 == val_last_1
        && val_last_1 == val
        && val != val_1 {
            let buffer = run_identical(&verbatim);
            compress_data.extend_from_slice(&buffer);
            verbatim = vec![];
            continue;
        }

        if i > 4  && i < data.len() - 2
           && is_identiqual(&vec![data[i - 3], data[i - 4], data[i - 5]])
           && val_last_1 == val
           && val != val_1
        {
            let buffer = run_identical(&verbatim);
            compress_data.extend_from_slice(&buffer);
            verbatim = vec![];
            continue;
        }

        if i < data.len() - 2
        && val != val_1
        && val_1 == val_2
        && val_2 == val_3 {
            let buffer = run_diff(&verbatim);
            compress_data.extend_from_slice(&buffer);
            verbatim = vec![];
            continue;
        }
    }
    verbatim.push(data[data.len() - 1]);
    let val = verbatim[verbatim.len() - 1];
    let mut val_last_1 = val;
    let mut val_last_2 = val;
    let mut val_last_3 = val;
    if verbatim.len() > 1 {
        val_last_1 = verbatim[verbatim.len() - 2];
    }
    if verbatim.len() > 2 {
        val_last_2 = verbatim[verbatim.len() - 3];
    }
    if verbatim.len() > 3 {
        val_last_3 = verbatim[verbatim.len() - 4];
    }
    let mut buffer;

    if verbatim.len() >= 2 && val == val_last_1 && val_last_1 == val_last_2 {
        if data.len() == 1 {
            buffer = vec![0, verbatim[0]];
        } else {
            buffer = run_identical(&verbatim);
        }
    } else if verbatim.len() >= 2 && val != val_last_1 && val_last_1 == val_last_2 && val_last_2 == val_last_3 {
        buffer = run_identical(&verbatim[..verbatim.len() - 1]);
        buffer.push(0);
        buffer.push(val);
    } else if verbatim.len() >= 2 && val == val_last_1 {
        buffer = run_diff(&verbatim[..verbatim.len() - 2]);
        buffer.push(1);
        buffer.push(verbatim[verbatim.len() - 1]);
    } else {
        buffer = run_diff(&verbatim);
    }

    compress_data.extend_from_slice(&buffer);
    compress_data
}

pub fn rle_decompress(data: &Vec<u8>) -> Vec<u8> {
    let mut decompress_data = vec![];

    let mut is_short_run_len_identical = false;
    let mut short_run_len_inc = 0;

    let mut is_long_run_identical = false;
    let mut wait_long_run_inc = 0;
    let mut long_run_identical_len = 0;

    let mut diff_buff = vec![];
    let mut is_short_run_len_diff = false;
    let mut short_run_len = 0;

    let mut is_long_run_diff = false;
    let mut long_run_diff_len:u16 = 0;

    let mut inc = 0;
    for byte in data {
        inc += 1;
        if is_short_run_len_identical {
            decompress_data.extend_from_slice(&vec![*byte; short_run_len_inc + 1]);
            is_short_run_len_identical = false;
            continue;
        }
        if is_long_run_identical {
            if long_run_identical_len == 0 {
                long_run_identical_len = *byte as u16 * 256 + data[inc] as u16;
                wait_long_run_inc = 2;
                continue;
            }
            wait_long_run_inc -= 1;
            if wait_long_run_inc == 0 {
                decompress_data.extend_from_slice(&vec![*byte; long_run_identical_len as usize]);
                long_run_identical_len = 0;
                wait_long_run_inc = 0;
                is_long_run_identical = false;
            }
            continue;
        }
        diff_buff.push(*byte);
        if is_short_run_len_diff && short_run_len != 0 {
            short_run_len -= 1;
            if short_run_len == 0 {
                decompress_data.extend_from_slice(&diff_buff);
                is_short_run_len_diff = false;
            }
            continue;
        }
        if is_long_run_diff {
            if long_run_diff_len == 0 {
                long_run_diff_len = *byte as u16 * 256 + data[inc] as u16 + 1;
                continue;
            }
            long_run_diff_len -= 1;
            if long_run_diff_len == 0 {
                decompress_data.extend_from_slice(&diff_buff[2..]);
                is_long_run_diff = false;
            }
            continue;
        }

        if *byte <= 126 {
            is_short_run_len_identical = true;
            short_run_len_inc = *byte as usize;
            continue;
        }
        if *byte == 127 {
            is_long_run_identical = true;
            continue;
        }

        diff_buff = vec![];
        if *byte == 128 {
            is_long_run_diff = true;
        }
        if *byte >= 129 {
            is_short_run_len_diff = true;
            short_run_len = 255 - *byte + 1;
        }
    }
    decompress_data
}
