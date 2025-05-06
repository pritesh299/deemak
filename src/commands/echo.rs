pub fn echo(args: &[&str]) -> String {
    if args.is_empty() {
        return String::new();
    }
    args.join(" ")
}
