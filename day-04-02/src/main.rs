mod data;

use data::INPUT as charset;

const WORD_COLUMNS: usize = 3;
const WORD_LINES: usize = 3;

fn main() {
    // Accomodate for all the unwanted line breaks and the one that is missing at the end.
    let charset_len = charset.len();
    let num_columns = charset
        .into_iter()
        .position(|&character| character == b'\n')
        .unwrap_or(charset_len) + 1;
    let last_column = num_columns - WORD_COLUMNS;

    let num_lines = charset_len / num_columns;
    let last_line = num_lines - WORD_LINES;

    let mut sum = 0;

    // Column loop is exclusive ("..") because last_column als includes line breaks.
    for column_index in 0..last_column {
        // This loop is inclusive ("..=") because last_line doesn't have an equivalent to line breaks.
        for line_index in 0..=last_line {
            let index = line_index * num_columns + column_index;
            let character_tl = charset[index];
            let character_tr = charset[index + 2];
            let character_c = charset[index + num_columns + 1];
            let character_bl = charset[index + 2 * num_columns];
            let character_br = charset[index + 2 * num_columns + 2];

            if character_c == b'A'
                && (((character_tl == b'M' && character_br == b'S')
                    && ((character_tr == b'M' && character_bl == b'S')
                        || (character_tr == b'S' && character_bl == b'M')))
                    || ((character_tl == b'S' && character_br == b'M')
                        && ((character_tr == b'M' && character_bl == b'S')
                            || (character_tr == b'S' && character_bl == b'M'))))
            {
                sum += 1;
            }
        }
    }

    println!("Recognitions: {}", sum);
}
