proto_dir := "./crates/zitadel-gen/src/api/"
gen_dir := "./crates/zitadel-gen"
zitadel_proto_version := "v4.12.2"


default: clean generate-grpc

clean:
    @rm -rf {{proto_dir}}

generate-grpc:
    buf generate https://github.com/zitadel/zitadel.git#tag={{zitadel_proto_version}} --path ./proto/zitadel

build_zitadel-grpc:
    cargo set-version --package zitadel-gen `echo {{zitadel_proto_version}} | sed 's/^v//'`
    cargo build --package zitadel-gen --release

install-tools:
    cargo install protoc-gen-prost-crate@0.5.0 cargo-edit

all_examples:
    # fetch_profile_with_pat
    cargo run -p zitadel-examples --example fetch_profile_with_pat \
      --features "zitadel/api zitadel/interceptors tokio" --release

    # fetch_profile_with_service_account
    cargo run -p zitadel-examples --example fetch_profile_with_service_account \
      --features "zitadel/api zitadel/interceptors tokio" --release

    # axum_webapi_oauth_interception_basic
    cargo run -p zitadel-examples --example axum_webapi_oauth_interception_basic \
      --features "axum" --release

    # service_account_authentication
    cargo run -p zitadel-examples --example service_account_authentication --features "zitadel/credentials tokio" --release