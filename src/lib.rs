use std::collections::HashMap;

pub fn run(n: u64, padding: usize, width: usize, height: usize) -> String {
    let mut result = String::new();

    let rows = as_rows(n, padding, char_map());

    // Fill the first row with each number's "cap".
    result.push_str(&top_row(&rows[0], width));

    // Then, the next rows are variable in height, but only the last
    // subrow of each one of them must contain horizontal separators.
		variable_height_row(&mut result, &rows[1], width, height);
		variable_height_row(&mut result, &rows[2], width, height);

    result
}

fn variable_height_row(result: &mut String, row: &[bool], width: usize, height: usize) {
    for _ in 0..height - 1 {
        result.push_str(&middle_row(&row, width));
    }
    result.push_str(&bottom_row(&row, width));
}

fn as_rows(n: u64, padding: usize, map: HashMap<String, u8>) -> [Vec<bool>; 3] {
    pad_input(n, padding)
        .chars()
        .map(|c| map.get(&c.to_string()).unwrap())
        .map(bits_to_vec)
        .fold([Vec::new(), Vec::new(), Vec::new()], build_rows)
}

fn build_rows(mut lines: [Vec<bool>; 3], bits: [bool; 8]) -> [Vec<bool>; 3] {
    lines[0].push(bits[0]);
    lines[1].push(bits[1]);
    lines[1].push(bits[2]);
    lines[1].push(bits[3]);
    lines[2].push(bits[4]);
    lines[2].push(bits[5]);
    lines[2].push(bits[6]);
    lines
}

fn char_map() -> HashMap<String, u8> {
    let mut map: HashMap<String, u8> = HashMap::new();
    map.insert("0".to_string(), 0b11011110u8);
    map.insert("1".to_string(), 0b00010010u8);
    map.insert("2".to_string(), 0b10111100u8);
    map.insert("3".to_string(), 0b10110110u8);
    map.insert("4".to_string(), 0b01110010u8);
    map.insert("5".to_string(), 0b11100110u8);
    map.insert("6".to_string(), 0b11101110u8);
    map.insert("7".to_string(), 0b10010010u8);
    map.insert("8".to_string(), 0b11111110u8);
    map.insert("9".to_string(), 0b11110110u8);
    map
}

fn pad_input(n: u64, padding: usize) -> String {
    let input = format!("{}", n);
    if input.len() >= padding {
        return input;
    }
    let missing = padding - input.len();
    let mut result = String::new();
    for _ in 0..missing {
        result.push('0');
    }
    result.push_str(&input);
    result
}

fn bits_to_vec(n: &u8) -> [bool; 8] {
    let mut result: [bool; 8] = [false; 8];
    let mut current = n.clone();
    for i in 0..8 {
        // Shifting the bits to the right reverses
        // the order because of endianness.
        result[7 - i] = (current & 0x01) != 0;
        current = current >> 1;
    }
    result
}

fn row_with_separator(bits: &[bool], width: usize, sep: bool) -> String {
    assert_eq!(0, bits.len() % 3);
    let mut tmp = String::new();
    for i in 0..bits.len() {
        match i % 3 {
            0 | 2 => tmp.push_str(&vertical(bits[i])),
            _ => tmp.push_str(&horizontal(sep && bits[i], width)),
        };
    }
    let mut trimmed = tmp.trim_end().to_string();
    trimmed.push('\n');
    trimmed
}

fn top_row(bits: &[bool], width: usize) -> String {
    row_with_separator(
        &bits
            .into_iter()
            .flat_map(|&has_cap| vec![false, has_cap, false])
            .map(|b| b.clone())
            .collect::<Vec<bool>>(),
        width,
        true,
    )
}

fn middle_row(bits: &[bool], width: usize) -> String {
    row_with_separator(bits, width, false)
}

fn bottom_row(bits: &[bool], width: usize) -> String {
    row_with_separator(bits, width, true)
}

fn horizontal(yes: bool, width: usize) -> String {
    (0..width).map(|_| if yes { '_' } else { ' ' }).collect()
}

fn vertical(yes: bool) -> String {
    if yes {
        String::from("|")
    } else {
        String::from(" ")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bits_to_vec() {
        assert_eq!(
            [true, true, false, false, false, false, false, true],
            bits_to_vec(&0b11000001)
        );
    }

    #[test]
    fn test_vertical_should_be_one_pipe_if_yes() {
        let expected = String::from("|");
        let actual = vertical(true);
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_vertical_should_be_one_space_if_no() {
        let expected = String::from(" ");
        let actual = vertical(false);
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_horizontal_should_b_n_underscores_if_yes() {
        let expected = String::from("__");
        let actual = horizontal(true, 2);
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_horizontal_should_b_n_spaces_if_no() {
        let expected = String::from("   ");
        let actual = horizontal(false, 3);
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_row_should_be_a_single_carriage_return_if_all_false() {
        let expected = String::from("\n");
        let actual = row_with_separator(&vec![false, false, false], 1, true);
        assert_eq!(expected, actual);
    }

    #[test]
    #[should_panic]
    fn test_row_should_panic_if_len_not_multiple_of_3() {
        row_with_separator(&vec![false, false], 1, true);
    }

    #[test]
    fn test_every_first_and_last_char_in_each_tuple_is_vertical() {
        let expected = String::from("| || |\n");
        let actual = row_with_separator(&vec![true, false, true, true, false, true], 1, true);
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_every_middle_char_in_each_tuple_is_horizontal() {
        let expected = String::from(" _  _\n");
        let actual = row_with_separator(&vec![false, true, false, false, true, false], 1, true);
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_horizontal_separators_can_be_forced_to_no() {
        let expected = String::from("| |\n");
        let actual = row_with_separator(&vec![true, true, true], 1, false);
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_padded_string() {
        let expected = String::from("01234");
        let actual = pad_input(1234, 5);
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_no_excessive_padding_should_be_applied() {
        let expected = String::from("1234");
        let actual = pad_input(1234, 4);
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_negative_padding() {
        let expected = String::from("1234");
        let actual = pad_input(1234, 3);
        assert_eq!(expected, actual);
    }
}
