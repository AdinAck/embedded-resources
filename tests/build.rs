fn main() {
    println!("cargo::rustc-check-cfg=cfg(bogus_flag, values(none()))");
}
