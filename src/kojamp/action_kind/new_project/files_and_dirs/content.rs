use crate::core::consts::program::{PROGRAM_REPO_URL, PROGRAM_VERSION};

pub fn java(class_name: &str) -> String {
    let comment_section = [
        format!("/**"),
        format!(" * This file was generated using kojamp CLI-app"),
        format!(
            " * Take a look at the official repository at {}",
            PROGRAM_REPO_URL
        ),
        format!(" */"),
    ]
    .join("\n");
    let class_begin = format!("public class {} {{", class_name);
    let turn_green_function = [
        "    private static String turnGreen(String text) {",
        "        return \"\\u001b[92m\" + text + \"\\u001b[0m\";",
        "    }",
    ]
    .join("\n");
    let println_function = [
        "    private static void println(Object o) {",
        "        System.out.println(o);",
        "    }",
    ]
    .join("\n");
    let print_function = [
        "    private static void print(Object o) {",
        "        System.out.print(o);",
        "    }",
    ]
    .join("\n");
    let greeting_function = [
        format!("    public String greeting() {{"),
        format!(
            "        return \"This is a hello from \" + turnGreen(\"'{}'\") + \" project!\";",
            class_name
        ),
        format!("}}"),
    ]
    .join("\n");
    let main_function = [
        format!("    public static void main(String[] args) {{"),
        format!("        print(\"Hi\");"),
        format!("        print(\" \");"),
        format!("        println(\"there!\");"),
        format!(""),
        format!("        println(new {}().greeting());", class_name),
        format!("    }}"),
    ]
    .join("\n");
    let class_end: String = "}".into();

    [
        comment_section,
        class_begin,
        turn_green_function,
        println_function,
        print_function,
        greeting_function,
        main_function,
        class_end,
    ]
    .join("\n\n")
}

pub fn kotlin(file_name: &str) -> String {
    let comment_section = [
        format!("/**"),
        format!(" * This file was generated using kojamp CLI-app"),
        format!(" * Take a look at the official repository at") + PROGRAM_REPO_URL,
        format!(" */"),
    ]
    .join("\n");
    let turn_green_function = [
        "fun turnGreen(value: String): String {",
        "    return \"\\u001b[92m$value\\u001b[0m\"",
        "}",
    ]
    .join("\n");
    let greeeting_function = [
        format!("fun greeting(): String{{"),
        format!(
            "    return \"Hello from ${{turnGreen(\"'{}'\")}} project\"",
            file_name
        ),
        format!("}}"),
    ]
    .join("\n");
    let main_function = [
        "fun main() {",
        "    print(\"Hi\")",
        "    print(\" \")",
        "    println(\"there\")",
        " ",
        "    println(greeting())",
        "}",
    ]
    .join("\n");

    [
        comment_section,
        turn_green_function,
        greeeting_function,
        main_function,
    ]
    .join("\n\n")
}

pub fn readme(name: &str, kind: &str, authors: Option<Vec<String>>) -> String {
    let kind = match kind {
        "java" => "java-orange",
        _ => "kotlin-blue",
    };
    let head = [
        format!("<div align=\"center\">"),
        format!(""),
        format!("{}", name),
        "=".repeat(name.len()),
        format!(""),
        format!("[![built in](https://img.shields.io/badge/built_in-kojamp_{}-blue?)](https://github.com/nasccped/kojamp)", PROGRAM_VERSION),
            format!("[![project kind](https://img.shields.io/badge/project_kind-{}?)](#)", kind),
        format!(""),
        format!("</div>"),
    ]
    .join("\n");

    let description = format!("Welcome to the **{}** project", name);
    let may_authors: String = if let Some(ref aut) = authors {
        let begin: String = " by:\n\n".into();
        let author_list = aut
            .iter()
            .map(|a| format!("- {}", a))
            .collect::<Vec<_>>()
            .join("\n");
        begin + author_list.as_ref()
    } else {
        "!".into()
    };

    [head, description + may_authors.as_ref()].join("\n\n")
}

pub fn gitignore() -> &'static str {
    "\
    # Ignore only the output bytecode content\n\
    out/"
}

pub fn toml(name: &str, kind: &str, authors: Option<Vec<String>>) -> String {
    format!(
        "\
        # This file was created using the kojamp-CLI app.\n\
        # Manual changes aren't encouraged!\n\
        # If you found any error or have trouble using it, consider\n\
        # opening the official repository:\n\
        #      https://github.com/nasccped/kojamp\n\
        \n\
        [project]\n\
        name = \"{}\"\n\
        kind = \"{}\"\n\
        {}",
        name,
        kind,
        if let Some(aut) = authors {
            format!("authors = [{}]", aut.join(", "))
        } else {
            "".into()
        }
    )
}
