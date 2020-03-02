pub fn build_th(input: &str, e: u8) -> String {
    match e {
        1 => format!("<th align=\"left\">{}</th>", input),
        2 => format!("<th align=\"right\">{}</th>", input),
        3 => format!("<th align=\"center\">{}</th>", input),
        _ => format!("<th>{}</th>", input),
    }
}

pub fn build_td(input: &str, e: u8) -> String {
    match e {
        1 => format!("<td align=\"left\">{}</td>", input),
        2 => format!("<td align=\"right\">{}</td>", input),
        3 => format!("<td align=\"center\">{}</td>", input),
        _ => format!("<td>{}</td>", input),
    }
}
