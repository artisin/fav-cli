use std::collections::HashMap;

use crate::args::Platform;

pub const MANIFEST: &str = "{
  \"icons\": [
    { \"src\": \"/192.png\", \"type\": \"image/png\", \"sizes\": \"192x192\" },
    { \"src\": \"/512.png\", \"type\": \"image/png\", \"sizes\": \"512x512\" }
  ]
}
";

pub fn generate_template(platforms: Vec<Platform>) -> String {
    let size_map: HashMap<Platform, String> = HashMap::from([
        (
            Platform::Web,
            "<link rel=\"icon\" href=\"/favicon.ico\" sizes=\"32x32\">".to_string(),
        ),
        (
            Platform::Modern,
            "<link rel=\"icon\" href=\"/icon.svg\" type=\"image/svg+xml\">".to_string(),
        ),
        (
            Platform::Apple,
            "<link rel=\"apple-touch-icon\" href=\"/apple-touch-icon.png\">".to_string(),
        ),
        (
            Platform::Android,
            "<link rel=\"manifest\" href=\"/manifest.webmanifest\">".to_string(),
        ),
    ]);

    let tags: Vec<String> = platforms
        .iter()
        .map(|p| size_map.get(p).unwrap().to_owned())
        .collect();

    format!(
        "<!DOCTYPE html>
<html>
<head>
  <meta charset=\"UTF-8\">
  <meta http-equiv=\"X-UA-Compatible\" content=\"IE=edge\">
  <meta name=\"viewport\" content=\"width=device-width, initial-scale=1.0\">
  <title>Sample Fav Project</title>
  {favicons}
</head>

<body>

</body>
</html>
",
        favicons = tags.join("\n  ")
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_template() {
        let platforms = vec![
            Platform::Web,
            Platform::Modern,
            Platform::Android,
            Platform::Apple,
        ];

        assert_eq!(
            "<!DOCTYPE html>
<html>
<head>
  <meta charset=\"UTF-8\">
  <meta http-equiv=\"X-UA-Compatible\" content=\"IE=edge\">
  <meta name=\"viewport\" content=\"width=device-width, initial-scale=1.0\">
  <title>Sample Fav Project</title>
  <link rel=\"icon\" href=\"/favicon.ico\" sizes=\"32x32\">
  <link rel=\"icon\" href=\"/icon.svg\" type=\"image/svg+xml\">
  <link rel=\"apple-touch-icon\" href=\"/apple-touch-icon.png\">
  <link rel=\"manifest\" href=\"/manifest.webmanifest\">
</head>

<body>

</body>
</html>
",
            generate_template(platforms)
        )
    }
}
