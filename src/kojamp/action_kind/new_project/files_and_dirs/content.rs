use crate::core::consts::program::{PROGRAM_REPO_URL, PROGRAM_VERSION};

fn comment_section() -> String {
    format!(
        "/**
 * This file was generated using kojamp CLI-app
 * Take a look at the official repository ({})
 */",
        PROGRAM_REPO_URL
    )
}

pub fn java(class_name: &str) -> String {
    format!(
        r#"{}

public class {} {{

    private static String turnGreen(String text) {{
        return "\u001b[92m" + text + "\u001b[0m";
    }}

    private static void println(Object o) {{
        System.out.println(o);
    }}

    private static String greenGreeting(String name) {{
        return "Hello, " + turnGreen(name) + "!";
    }}

    public static void main(String[] args) {{
        String projectName = "{}";
        println(greenGreeting(programName));
    }}

}}
"#,
        comment_section(),
        class_name,
        class_name
    )
}

pub fn kotlin(file_name: &str) -> String {
    format!(
        r#"{}

fun turnGreen(value: String): String {{
    return "\u001b[92m$value\u001b[0m"
}}

fun greenGreeting(name: String): String {{
    return "Hello, ${{turnGreen(name)}}!"
}}

fun main() {{
    val projectName = "{}"
    println(greenGreeting(projectName))
}}
"#,
        comment_section(),
        file_name
    )
}

pub fn readme(name: &str, kind: &str, authors: Option<Vec<String>>) -> String {
    let builtin_badge = format!(
        "[![built in](https://img.shields.io/badge/built_in-kojamp_{}-blue?)]({})",
        PROGRAM_VERSION, PROGRAM_REPO_URL
    );
    let project_kind_badge = format!(
        "[![project kind: {}](https://img.shields.io/badge/project_kind-{}?)](#)",
        kind,
        match kind {
            "java" => "java-orange",
            _ => "kotlin-blue",
        }
    );
    let may_authors: String = if let Some(ref aut) = authors {
        let begin = String::from(" by:\n\n");
        let authors_list = aut
            .iter()
            .map(|a| format!("- {}", a))
            .collect::<Vec<_>>()
            .join("\n");
        begin + authors_list.as_ref()
    } else {
        "!".into()
    };

    format!(
        r#"<div align="center">

{}
{}

{}
{}

</div>

Welcome to the **{}** project{}"#,
        name,
        "=".repeat(name.len()),
        builtin_badge,
        project_kind_badge,
        name,
        may_authors
    )
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
