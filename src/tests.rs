use super::*;

#[test]
fn cli_yaml_valid() {
    let cli_yaml = load_yaml!("../cli.yml");
    let _ = App::from_yaml(cli_yaml)
        .version(version!())
        .get_matches_from(vec!["tests"]);
}
