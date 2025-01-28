fn main() {
    //let url =
    //    "https://raw.githubusercontent.com/MyJetTools/settings-service/refs/heads/main/proto/";
    //ci_utils::sync_and_build_proto_file(url, "TemplatesService.proto");
    //ci_utils::sync_and_build_proto_file(url, "SecretsService.proto");

    ci_utils::tonic_build::compile_protos("./proto/SecretsService.proto").unwrap();
    ci_utils::tonic_build::compile_protos("./proto/TemplatesService.proto").unwrap();

    //ci_utils::sync_and_build_proto_file(url, "DomainsService.proto");
}
