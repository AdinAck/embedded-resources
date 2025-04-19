fn main() {
    let features = [
        cfg!(feature = "_test"),
        cfg!(feature = "stm32"),
        cfg!(feature = "nrf"),
    ];

    if !features.iter().any(|enabled| *enabled) {
        println!("cargo::error=Exactly one ecosystem feature must be specified.");
    } else if features.iter().filter(|enabled| **enabled).count() > 1 {
        println!("cargo::error=Exactly one ecosystem feature may be enabled at a time.");
    }
}
