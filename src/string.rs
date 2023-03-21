const CYAN: &str = "\u{1b}[36m";
const BOLD: &str = "\u{1b}[1m";
const RESET: &str = "\u{1b}[0m";

pub(crate) trait StringExt {
    fn cyan(self) -> Self;
    fn bold(self) -> Self;
}

impl StringExt for String {
    fn cyan(self) -> Self {
        format!("{}{}{}", CYAN, self, RESET)
    }

    fn bold(self) -> Self {
        format!("{}{}{}", BOLD, self, RESET)
    }
}
