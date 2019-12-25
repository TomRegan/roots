use std::collections::HashMap;
use regex::Regex;

fn clean_path(replacements: &HashMap<String, String>, path: String) -> String {
    let mut result = path.clone();
    for (pat, rep) in replacements.iter() {
        let rx = Regex::new(pat).unwrap();
        result = rx.replace_all(result.as_str(), rep.as_str()).to_string();
    }
    return result;
}


#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn path_is_corrected() {
        let replacements = [(r#"[<>:"\?!\*\|]"#.to_string(), r#"_"#.to_string()),
                            ("[/]".to_string(), "_".to_string()),
                            ("[\u{00}-\u{1f}]".to_string(), "".to_string()),
                            (r#"\.$"#.to_string(), r#"_"#.to_string()),
                            (r#"\s+$"#.to_string(), r#""#.to_string()),
                            (r#"^\."#.to_string(), r#"_"#.to_string())];
        let changes = [("Space_ The Final Frontier", "Space: The Final Frontier"),
                       ("Spaces, The Final Frontier", "Spaces, The Final Frontier   "),
                       ("_invisible", ".invisible"),
                       ("visible_", "visible."),
                       ("windows___nix_", "windows<>*nix?"),
                       ("put _that_ in your _", "put _that_ in your |"),
                       ("valid.epub", "valid.epub"),
                       ("_invalid_.epub", ".invalid!.epub")];
        for (expected, input) in changes.iter() {
            let result = clean_path(&replacements.iter().cloned().collect(), input.to_string());
            assert_eq!(expected.to_string(), result);
        }
    }

}

