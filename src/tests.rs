use super::*;

#[test]
fn yaml_valid() {
    let cli_yaml = load_yaml!("../cli.yml");
    let _ = App::from_yaml(cli_yaml)
        .version(version!())
        .get_matches_from(vec!["help"]);
    let interactive_yaml = load_yaml!("../interactive.yml");
    let _ = App::from_yaml(interactive_yaml)
        .version(version!())
        .get_matches_from(vec!["help"]);
}
