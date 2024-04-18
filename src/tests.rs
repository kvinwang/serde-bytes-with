#[test]
fn test_expand() {
    let stream = crate::patch(
        syn::parse_quote!("as_hexed_bytes"),
        syn::parse_quote! {
            #[derive(Serialize, Deserialize, TypeInfo)]
            #[derive(Clone, PartialEq, Message)]
            pub struct Attestation {
                #[prost(int32, tag = "1")]
                pub version: i32,
                #[prost(string, tag = "2")]
                pub provider: ::prost::alloc::string::String,
                #[prost(message, optional, tag = "3")]
                pub payload: ::core::option::Option<AttestationReport>,
                #[prost(bytes = "vec", tag = "5")]
                pub encoded_report: ::prost::alloc::vec::Vec<u8>,
                #[prost(bytes = "vec", tag = "6")]
                pub timestamp: ::prost::alloc::vec::Vec<u8>,
            }
        },
    );
    insta::assert_snapshot!(rustfmt_snippet::rustfmt_token_stream(&stream).unwrap())
}
