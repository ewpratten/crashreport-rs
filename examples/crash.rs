use crashreport::crashreport;

pub fn main() {
    crashreport!();

    panic!("This is a panic!");
}
