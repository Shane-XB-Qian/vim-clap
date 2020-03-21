lazy_static! { pub static ref EXACTMATCH_MAP: HashMap<&'static str, &'static str> = [("ai", ""),("awk", ""),("bash", ""),("bat", ""),("bin", ""),("bmp", ""),("c", ""),("cc", ""),("cfg", ""),("clj", ""),("cljc", ""),("cljs", ""),("coffee", ""),("conf", ""),("cp", ""),("cpp", ""),("csh", ""),("css", ""),("d", ""),("dart", ""),("db", ""),("diff", ""),("dump", ""),("dylib", ""),("edn", ""),("eex", ""),("ejs", ""),("erl", ""),("ex", ""),("exs", ""),("fish", ""),("fs", ""),("fsi", ""),("fsx", ""),("gif", ""),("go", ""),("gz", ""),("h", ""),("hbs", ""),("hpp", ""),("hrl", ""),("hs", ""),("htm", ""),("html", ""),("ico", ""),("ini", ""),("java", ""),("jl", ""),("jpeg", ""),("jpg", ""),("js", ""),("json", ""),("jsx", ""),("ksh", ""),("less", ""),("lhs", ""),("lock", ""),("log", ""),("lua", ""),("markdown", ""),("md", ""),("ml", "λ"),("mli", "λ"),("mustache", ""),("php", ""),("pl", ""),("plist", "况"),("pm", ""),("png", ""),("pp", ""),("ps1", ""),("psb", ""),("psd", ""),("py", ""),("pyc", ""),("pyd", ""),("pyo", ""),("rb", ""),("rlib", ""),("rmd", ""),("rmeta", ""),("rs", ""),("rss", ""),("sass", ""),("scala", ""),("scss", ""),("sh", ""),("slim", ""),("sln", ""),("so", ""),("sql", ""),("styl", ""),("suo", ""),("swift", ""),("t", ""),("timestamp", "﨟"),("toml", ""),("ts", ""),("tsx", ""),("twig", ""),("txt", ""),("vim", ""),("vimrc", ""),("vue", "﵂"),("xcplayground", ""),("xul", ""),("yaml", ""),("yml", ""),("zip", ""),("zsh", "")].iter().copied().collect(); pub static ref EXTENSION_MAP: HashMap<&'static str, &'static str> = [("ai", ""),("awk", ""),("bash", ""),("bat", ""),("bin", ""),("bmp", ""),("c", ""),("cc", ""),("cfg", ""),("clj", ""),("cljc", ""),("cljs", ""),("coffee", ""),("conf", ""),("cp", ""),("cpp", ""),("csh", ""),("css", ""),("d", ""),("dart", ""),("db", ""),("diff", ""),("dump", ""),("dylib", ""),("edn", ""),("eex", ""),("ejs", ""),("erl", ""),("ex", ""),("exs", ""),("fish", ""),("fs", ""),("fsi", ""),("fsx", ""),("gif", ""),("go", ""),("gz", ""),("h", ""),("hbs", ""),("hpp", ""),("hrl", ""),("hs", ""),("htm", ""),("html", ""),("ico", ""),("ini", ""),("java", ""),("jl", ""),("jpeg", ""),("jpg", ""),("js", ""),("json", ""),("jsx", ""),("ksh", ""),("less", ""),("lhs", ""),("lock", ""),("log", ""),("lua", ""),("markdown", ""),("md", ""),("ml", "λ"),("mli", "λ"),("mustache", ""),("php", ""),("pl", ""),("plist", "况"),("pm", ""),("png", ""),("pp", ""),("ps1", ""),("psb", ""),("psd", ""),("py", ""),("pyc", ""),("pyd", ""),("pyo", ""),("rb", ""),("rlib", ""),("rmd", ""),("rmeta", ""),("rs", ""),("rss", ""),("sass", ""),("scala", ""),("scss", ""),("sh", ""),("slim", ""),("sln", ""),("so", ""),("sql", ""),("styl", ""),("suo", ""),("swift", ""),("t", ""),("timestamp", "﨟"),("toml", ""),("ts", ""),("tsx", ""),("twig", ""),("txt", ""),("vim", ""),("vimrc", ""),("vue", "﵂"),("xcplayground", ""),("xul", ""),("yaml", ""),("yml", ""),("zip", ""),("zsh", "")].iter().copied().collect(); }