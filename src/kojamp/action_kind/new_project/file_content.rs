use super::{ProjectFields, ProjectKind};
use crate::globals::{PROGRAM_REPO_URL, PROGRAM_VERSION};

pub fn java(class_name: &str) -> String {
    [
        "/**",
        " * This file was generated using kojamp CLI-app",
        format!(
            " * Take a look at the official repository at {}",
            PROGRAM_REPO_URL
        )
        .as_str(),
        " */",
        "",
        format!("public class {} {{", class_name).as_str(),
        "",
        "    private static String turnGreen(String text) {",
        "        return \"\\u001b[92m\" + text + \"\\u001b[0m\";",
        "    }",
        "",
        "    private static void println(Object o) {",
        "        System.out.println(o);",
        "    }",
        "",
        "    private static void println() {",
        "        System.out.println();",
        "    }",
        "",
        "    private static void print(Object o) {",
        "        System.out.print(o);",
        "    }",
        "",
        "    public String greeting() {",
        format!(
            "        return \"This is a hello from \" + turnGreen(\"'{}'\") + \" project!\";",
            class_name
        )
        .as_str(),
        "    }",
        "",
        "    public static void main(String[] args) {",
        "        print(\"Hi\");",
        "        print(\" \");",
        "        println(\"there!\");",
        "",
        format!("        println(new {}().greeting());", class_name).as_str(),
        "    }",
        "}",
    ]
    .join("\n")
}

pub fn kotlin(file_name: &str) -> String {
    [
        "/**",
        " * This file was generated using kojamp CLI-app,",
        format!(
            " * Take a look at the official repository at {}",
            PROGRAM_REPO_URL
        )
        .as_str(),
        " */",
        "",
        "fun turnGreen(value: String): String {",
        "    return \"\\u001b[92m$value\\u001b[0m\"",
        "}",
        "",
        "fun greeting(): String {",
        format!(
            "     return \"Hello from ${{turnGreen(\"'{}'\")}} project\"",
            file_name
        )
        .as_str(),
        "}",
        "",
        "fun main() {",
        "    print(\"Hi\")",
        "    print(\" \")",
        "    println(\"there\")",
        "",
        "    println(greeting())",
        "}",
    ]
    .join("\n")
}

pub fn readme(project_fields: &ProjectFields) -> String {
    let name = project_fields.get_name().get_inner();
    let kind = project_fields.get_kind();
    let authors = project_fields.get_authors().get_inner();

    let mut begin = vec![
        String::from("<div align=\"center\">"),
        String::new(),
        format!("{}", name),
        format!("{}", "=".repeat(name.len())),
        String::new(),
        format!("[![built in](https://img.shields.io/badge/built_in-kojamp_{}-blue?)](https://github.com/nasccped/kojamp)", PROGRAM_VERSION),
        format!(
            "[![project kind](https://img.shields.io/badge/project_kind-{}?)](#)", match kind {
                ProjectKind::Java => "java-orange",
                _ => "kotlin-blue",
            }
        ),
        String::new(),
        String::from("</div>"),
        String::new(),
        format!(
            "Welcome to the **{}** project{}",
            name,
            if authors.is_some() {
                " by:\n"
            } else {
                "!"
            }
        ),
    ];

    let authors = if let Some(aut) = authors {
        aut.iter()
            .map(|a| format!("- {}", a))
            .collect::<Vec<_>>()
            .join("\n")
    } else {
        String::from("")
    };
    begin.push(authors);
    begin.join("\n")
}

pub fn gitignore() -> String {
    ["# Ignore only the output bytecode content", "out/"].join("\n")
}
