use std::path::Path;

pub fn detect_language(path: &Path) -> String {
    match path.extension().and_then(|s| s.to_str()).unwrap_or("").to_lowercase().as_str() {
        // General languages
        "rs" => "Rust".to_string(),
        "py" => "Python".to_string(),
        "go" => "Go".to_string(),
        "js" => "JavaScript".to_string(),
        "ts" => "TypeScript".to_string(),
        "java" => "Java".to_string(),
        "c" => "C".to_string(),
        "cpp" | "cc" | "cxx" => "C++".to_string(),
        "cs" => "C#".to_string(),
        "rb" => "Ruby".to_string(),
        "php" => "PHP".to_string(),
        "swift" => "Swift".to_string(),
        "kt" | "kts" => "Kotlin".to_string(),

        // Web languages
        "html" => "HTML".to_string(),
        "css" => "CSS".to_string(),
        "scss" => "Sass/SCSS".to_string(),
        "less" => "Less".to_string(),
        "jsx" => "React/JSX".to_string(),
        "vue" => "Vue.js".to_string(),

        // Data languages
        "r" => "R".to_string(),
        "sql" => "SQL".to_string(),
        "md" => "Markdown".to_string(),
        "yaml" | "yml" => "YAML".to_string(),

        // Other languages
        "clj" => "Clojure".to_string(),
        "elixir" => "Elixir".to_string(),
        "lua" => "Lua".to_string(),
        "dart" => "Dart".to_string(),
        "haskell" => "Haskell".to_string(),
        "groovy" => "Groovy".to_string(),
        "objective-c" | "m" => "Objective-C".to_string(),
        "f" => "F#".to_string(),
        "delphi" | "pas" => "Delphi/Object Pascal".to_string(),
        "v" => "V".to_string(),
        "nim" => "Nim".to_string(),

        // Template languages
        "twig" => "Twig".to_string(),
        "mustache" => "Mustache".to_string(),

        // Documentation/markup languages
        "rst" => "reStructuredText".to_string(),
        "tex" | "latex" => "LaTeX".to_string(),
        "asciidoc" => "AsciiDoc".to_string(),

        // Unrecognized extensions
        _ => "unknown".to_string(),  // Handle all other cases
    }
}