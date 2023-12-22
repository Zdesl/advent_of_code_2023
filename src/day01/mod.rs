use std::io::BufRead;

const STR_NUMS: [&str; 9] = ["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];

fn first_match(char_array: std::str::Chars<'_>) -> u8 {
    let mut line: Vec<char> = char_array.clone().collect();
    const WIN_SZ: usize = 5;
    let pad = vec!['?';WIN_SZ - 1];
    line.extend(pad);
    for win in line.windows(WIN_SZ) {
        if win[0].is_ascii_digit() {
            return win[0].to_digit(10).unwrap_or(0) as u8
        }
        for (i, strnum) in STR_NUMS.iter().enumerate() {
            for (j, strnumchar) in strnum.chars().enumerate() {
                if j >= WIN_SZ || win[j] != strnumchar {
                    break;
                }
                if j == strnum.chars().count() - 1 {
                    return (i + 1) as u8;
                }
            }
        }
    }
    0
}

fn last_match(char_array: std::str::Chars<'_>) -> u8 {
    let mut line: Vec<char> = char_array.clone().rev().collect();
    const WIN_SZ: usize = 5;
    let pad = vec!['?';WIN_SZ - 1];
    line.extend(pad);
    for win in line.windows(WIN_SZ) {
        if win[0].is_ascii_digit() {
            return win[0].to_digit(10).unwrap_or(0) as u8
        }
        for (i, strnum) in STR_NUMS.iter().rev().enumerate() {
            for (j, strnumchar) in strnum.chars().rev().enumerate() {
                if j >= WIN_SZ || win[j] != strnumchar {
                    break;
                }
                if j == strnum.chars().count() - 1 {
                    return 9 - i as u8;
                }
            }
        }
    }
    0
}

pub fn s1() {
    let file = std::fs::OpenOptions::new().read(true).open("src\\day01\\input").unwrap();
    let mut reader = std::io::BufReader::with_capacity(128, file);
    let buffer: &mut String = &mut String::new();
    let mut n_bytes_read;
    let mut total: u32 = 0;
    loop {
        n_bytes_read = reader
            .read_line(buffer)
            .unwrap();
        if n_bytes_read == 0 {
            break;
        };

        let lnum: u8;
        if let Some(idx_lnum) = buffer.find(char::is_numeric) {
            lnum = (buffer.as_bytes()[idx_lnum] as char).to_digit(10).unwrap() as u8;
        } else {
            lnum = 0;
        }
        let rnum: u8;
        if let Some(idx_rnum) = buffer.rfind(char::is_numeric) {
            rnum = (buffer.as_bytes()[idx_rnum] as char).to_digit(10).unwrap() as u8;
        } else {
            rnum = 0;
        }
        total += ((lnum * 10) + rnum) as u32;

        buffer.clear();
    }
    println!("d01s1={}", total);
}

pub fn s2() {
    let file = std::fs::OpenOptions::new().read(true).open("src\\day01\\input").unwrap();
    let mut reader = std::io::BufReader::with_capacity(128, file);
    let buffer: &mut String = &mut String::new();
    let mut n_bytes_read;
    let mut total: u32 = 0;
    loop {
        n_bytes_read = reader
            .read_line(buffer)
            .unwrap();
        if n_bytes_read == 0 {
            break
        };
        let chars = buffer.strip_suffix("\r\n").unwrap().chars();

        let first_match = first_match(chars.clone());
        let last_match = last_match(chars);
        total += (first_match as u32 * 10) + (last_match as u32);

        buffer.clear();
    }
    println!("d01s2={}", total);
}
