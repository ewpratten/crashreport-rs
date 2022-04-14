use crashreport::enable_issue_tracking;

pub fn main() {
    enable_issue_tracking!();

    panic!("This is a panic!");
}
