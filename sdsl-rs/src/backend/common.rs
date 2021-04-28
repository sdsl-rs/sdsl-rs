pub static ENV_SKIP_BUILD: &str = "SKIP_SDSL_BUILD";

pub fn skip_build() -> bool {
    return std::env::var(ENV_SKIP_BUILD).unwrap_or("0".to_string()) == "1";
}
