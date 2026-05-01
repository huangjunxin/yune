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
