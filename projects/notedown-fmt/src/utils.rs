pub use textwrap::{dedent, indent};

pub fn dedent_less_than(input: &str, indent: usize) -> String {
    let mut out: String = String::from(input);
    let mut j: usize = 0;
    let mut is_counting = true;
    let mut ws_cnt = 0;
    unsafe {
        let out_b = out.as_bytes_mut();
        for i in 0..out_b.len() {
            if is_counting == true && out_b[i] == b' ' {
                ws_cnt += 1;
                if ws_cnt == indent {
                    is_counting = false;
                }
            } else {
                is_counting = false;
                if out_b[i] == b'\n' {
                    is_counting = true;
                    ws_cnt = 0;
                }
                out_b[j] = out_b[i];
                j += 1;
            }
        }
    }
    out.truncate(j);
    out
}
