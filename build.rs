fn main() {
    //let url =
    //    "https://raw.githubusercontent.com/MyJetTools/settings-service/refs/heads/main/proto/";
    //ci_utils::sync_and_build_proto_file(url, "TemplatesService.proto");
    //ci_utils::sync_and_build_proto_file(url, "SecretsService.proto");

    ci_utils::compile_protos("./proto/SecretsService.proto");
    ci_utils::compile_protos("./proto/TemplatesService.proto");

    //ci_utils::sync_and_build_proto_file(url, "DomainsService.proto");
}
