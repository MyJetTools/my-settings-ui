fn main() {
    let url = "https://raw.githubusercontent.com/MyJetTools/settings-service/main/proto/";
    ci_utils::sync_and_build_proto_file(url, "TemplatesService.proto");
    ci_utils::sync_and_build_proto_file(url, "SecretsService.proto");
    ci_utils::sync_and_build_proto_file(url, "DomainsService.proto");
}
