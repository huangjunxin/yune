#[path = "frontend_hosts/mod.rs"]
mod frontend_hosts;

#[test]
fn frontend_host_trace_fixture_contract_is_sanitized() {
    frontend_hosts::assert_baseline_fixture_is_sanitized();
}

#[test]
fn typeduck_web_wrapper_lifecycle_is_validated_through_yune_abi() {
    frontend_hosts::typeduck_web::typeduck_web_wrapper_lifecycle_is_validated_through_yune_abi();
}

#[test]
fn typeduck_web_basic_fixture_is_sanitized_and_matches_trace_contract() {
    let fixture = std::fs::read_to_string("fixtures/frontend-traces/typeduck-web-basic.json")
        .expect("TypeDuck-Web basic trace fixture should exist");
    frontend_hosts::typeduck_web::assert_typeduck_web_fixture_contract(&fixture);
}
