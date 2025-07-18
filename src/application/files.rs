use regex::Regex;

use crate::filesystem::Replacements;

#[allow(unused)]
fn clean_path(replacements: Replacements, path: String) -> String {
    replacements.iter().fold(path, |acc, (pat, rep)| {
        Regex::new(pat)
            .unwrap()
            .replace_all(acc.as_str(), rep.as_str())
            .to_string()
    })
}

#[cfg(test)]
mod tests {
    extern crate rstest;

    use maplit::hashmap;

    use super::*;

    use self::rstest::*;

    #[fixture]
    fn replacements() -> Replacements {
        hashmap! {
            r#"[<>:"\?!\*\|/]"#.to_string() => r#"_"#.to_string(),
            "[\u{00}-\u{1f}]".to_string() => "".to_string(),
            r#"\.$"#.to_string() => r#"_"#.to_string(),
            r#"\s+$"#.to_string() => r#""#.to_string(),
            r#"^\s+"#.to_string() => r#""#.to_string(),
            r#"^\."#.to_string() => r#"_"#.to_string()
        }
    }

    #[rstest]
    #[case::paths("path/less/travelled", "path_less_travelled")]
    #[case::spaces("Space: The Final Frontier", "Space_ The Final Frontier")]
    #[case::trailing_spaces("Spaces, The Final Frontier   ", "Spaces, The Final Frontier")]
    #[case::initial_spaces("   Spaces, The Initial Frontier", "Spaces, The Initial Frontier")]
    #[case::non_printing(".invisible", "_invisible")]
    #[case::trailing_dot("visible.", "visible_")]
    #[case::punctuation("windows<>*nix?", "windows___nix_")]
    #[case::pipes("put _that_ in your |", "put _that_ in your _")]
    #[case::identity("valid.epub", "valid.epub")]
    #[case::exclaimation_marks(".invalid!.epub", "_invalid_.epub")]
    fn path_is_corrected(#[case] input: String, #[case] expected: String, replacements: Replacements) {
        let actual: String = clean_path(replacements, input);
        assert_eq!(expected, actual)
    }
}
