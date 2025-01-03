mod data;

struct Matcher {
    start: usize,
    offset: usize,
}

const WORD: &[u8] = b"XMAS";
const WORD_REVERSE: &[u8] = b"SAMX";
const WORD_SIZE: usize = WORD.len();

fn word_check_forward(current_pos: usize, matcher: &Matcher) -> bool {
    return word_check(current_pos, matcher, WORD);
}

fn word_check_reverse(current_pos: usize, matcher: &Matcher) -> bool {
    return word_check(current_pos, matcher, WORD_REVERSE);
}

fn word_check(current_pos: usize, matcher: &Matcher, word_array: &[u8]) -> bool {
    for i in 0..WORD_SIZE {
        let pos_in_array = matcher.start + current_pos + matcher.offset * i + i;
        if pos_in_array >= data::INPUT.len() {
            return false;
        }
        if data::INPUT[pos_in_array] != word_array[i] {
            return false;
        }
    }
    return true;
}

fn redundant_method() -> i32 {
    let num_columns = data::INPUT
        .iter()
        .position(|&character| character == b'\n')
        .unwrap();
    let matcher_horz = Matcher {
        start: 0,
        offset: 0,
    };
    let matcher_vert = Matcher {
        start: 0,
        offset: num_columns,
    };
    let matcher_diag = Matcher {
        start: 0,
        offset: num_columns + 1,
    };
    let matcher_diag2 = Matcher {
        start: WORD_SIZE - 1,
        offset: num_columns - 1,
    };

    let total_length = data::INPUT.len();
    let num_lines = total_length / num_columns;
    let end_pos = total_length - WORD_SIZE; //(num_lines - WORD_SIZE) * num_columns + (num_columns - WORD_SIZE);

    let mut num_words_horz_fwd = 0;
    let mut num_words_horz_rev = 0;
    let mut num_words_vert_fwd = 0;
    let mut num_words_vert_rev = 0;
    let mut num_words_diag_fwd = 0;
    let mut num_words_diag_rev = 0;
    let mut num_words_diag2_fwd = 0;
    let mut num_words_diag2_rev = 0;
    for i in 0..end_pos {
        // // Check horz
        if word_check_forward(i, &matcher_horz) {
            num_words_horz_fwd += 1;
        }
        if word_check_reverse(i, &matcher_horz) {
            num_words_horz_rev += 1;
        }
        // Check vert
        if word_check_forward(i, &matcher_vert) {
            num_words_vert_fwd += 1;
        }
        if word_check_reverse(i, &matcher_vert) {
            num_words_vert_rev += 1;
        }
        // Check diag
        if word_check_forward(i, &matcher_diag) {
            num_words_diag_fwd += 1;
        }
        if word_check_reverse(i, &matcher_diag) {
            num_words_diag_rev += 1;
        }
        // Check diag in other direction
        if word_check_forward(i, &matcher_diag2) {
            num_words_diag2_fwd += 1;
        }
        if word_check_reverse(i, &matcher_diag2) {
            num_words_diag2_rev += 1;
        }
    }

    println!("Num words horz fwd: {}", num_words_horz_fwd);
    println!("Num words horz rev: {}", num_words_horz_rev);
    println!("Num words vert fwd: {}", num_words_vert_fwd);
    println!("Num words vert rev: {}", num_words_vert_rev);
    println!("Num words diag fwd: {}", num_words_diag_fwd);
    println!("Num words diag rev: {}", num_words_diag_rev);
    println!("Num words diag2 fwd: {}", num_words_diag2_fwd);
    println!("Num words diag2 rev: {}", num_words_diag2_rev);
    println!("Line break at: {}", num_columns);
    println!("Num lines: {}", num_lines);

    return num_words_horz_fwd
        + num_words_horz_rev
        + num_words_vert_fwd
        + num_words_vert_rev
        + num_words_diag_fwd
        + num_words_diag_rev
        + num_words_diag2_fwd
        + num_words_diag2_rev;
}

fn main() {
    let num_words = redundant_method();
    // println!("{}", String::from_utf8_lossy(data::INPUT));
    println!("Number of words: {}", num_words);
}
