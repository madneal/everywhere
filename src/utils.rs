use std::path::Path;

pub fn should_skip_dir(name: &str) -> bool {
    matches!(
        name,
        ".git"
            | "target"
            | "node_modules"
            | ".idea"
            | "__pycache__"
            | ".svn"
            | ".hg"
            | "dist"
            | "build"
            | ".cargo"
            | "venv"
            | ".venv"
            | "site-packages"
            | ".next"
            | ".nuxt"
            | ".cache"
    )
}

pub fn is_probably_binary_extension(ext: &str) -> bool {
    matches!(
        ext.to_ascii_lowercase().as_str(),
        "png" | "jpg" | "jpeg" | "gif" | "webp" | "ico" | "bmp" | "tif" | "tiff"
            | "zip" | "gz" | "tar" | "rar" | "7z" | "bz2" | "xz"
            | "exe" | "dll" | "so" | "dylib" | "o" | "a" | "lib"
            | "bin" | "wasm" | "pyc" | "pyo" | "class" | "jar"
            | "mp3" | "mp4" | "mkv" | "avi" | "mov" | "wav" | "flac"
            | "ttf" | "otf" | "woff" | "woff2" | "eot"
            | "sqlite" | "db" | "mdb"
            | "pdf"
            | "docx"
    )
}

pub fn is_known_text_extension(ext: &str) -> bool {
    matches!(
        ext.to_ascii_lowercase().as_str(),
        "txt" | "rst" | "log"
            | "rs" | "toml" | "lock"
            | "json" | "yaml" | "yml" | "xml" | "html" | "htm" | "css" | "scss" | "sass" | "less"
            | "js" | "mjs" | "cjs" | "ts" | "tsx" | "jsx" | "vue" | "svelte"
            | "py" | "pyi" | "pyw" | "go" | "java" | "kt" | "kts" | "scala" | "clj" | "cljs"
            | "c" | "h" | "cc" | "cpp" | "cxx" | "hpp" | "hh" | "hxx"
            | "sh" | "bash" | "zsh" | "fish" | "ps1" | "bat" | "cmd"
            | "sql" | "graphql" | "proto"
            | "rb" | "php" | "pl" | "pm" | "r" | "lua" | "swift" | "m" | "mm"
            | "cs" | "fs" | "fsx" | "vb"
            | "tex" | "bib" | "org" | "csv" | "tsv" | "ini" | "cfg" | "conf" | "config"
            | "dockerfile" | "gitignore" | "editorconfig" | "env" | "properties"
            | "cmake" | "makefile" | "mk" | "gradle" | "sbt" | "nim" | "zig" | "v" | "dart"
            | "elm" | "ex" | "exs" | "erl" | "hrl" | "hs" | "lhs" | "ml" | "mli" | "fsi"
            | "jl" | "cr" | "pas" | "pp" | "lpr" | "d" | "ada" | "adb" | "ads"
            | "asm" | "s" | "S" | "lisp" | "cl" | "lsp" | "rkt" | "scm" | "ss"
            | "ipynb"
    )
}

pub fn looks_binary_sample(sample: &[u8]) -> bool {
    if sample.is_empty() {
        return true;
    }
    if sample.contains(&0) {
        return true;
    }
    let ctrl = sample
        .iter()
        .filter(|&&b| b < 9 || (b > 13 && b < 32))
        .count();
    ctrl * 10 > sample.len() * 3
}

pub fn file_base_name(path: &Path) -> String {
    path.file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or("unknown")
        .to_string()
}
