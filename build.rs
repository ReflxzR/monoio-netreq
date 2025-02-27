fn main() {
    let default_crate = cfg!(feature = "default-crate");
    let pool = cfg!(feature = "pool");
    let hyper_tls = cfg!(feature = "hyper-tls");

    if default_crate && (pool || hyper_tls) {
        panic!("'default-crate' feature cannot work with 'pool' or 'hyper-tls' feature");
    }

    if pool && hyper_tls {
        panic!("'pool' and 'hyper-tls' feature cannot work together");
    }

    if !default_crate && !pool && !hyper_tls {
        panic!("No features enabled! At least one of default-features, pool-patch or hyper-tls feature is required");
    }
}