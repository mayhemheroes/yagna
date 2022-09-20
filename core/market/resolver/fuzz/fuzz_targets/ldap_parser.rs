#![no_main]
use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &str| {
    // fuzzed code goes here
    _ = ya_market_resolver::resolver::ldap_parser::parse(data);
});
