extern crate pkg_config;

fn main() {
    println!("cargo:rustc-link-search=native=c-code/target");

    //Why did a add this
    println!("cargo:rustc-cdylib-link-arg=-undefined");
    println!("cargo:rustc-cdylib-link-arg=dynamic_lookup");

    //Link some of the installed c/c++ libs
    //pkg_config::Config::new().statik(true).probe("glib-2.0").unwrap();
    pkg_config::Config::new()
        .statik(true)
        .probe("arrow-glib")
        .unwrap();
    pkg_config::Config::new()
        .statik(true)
        .probe("arrow")
        .unwrap();

    //link in libtest_c.a
    println!("cargo:rustc-link-lib=static=c_test");
}
