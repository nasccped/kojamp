use super::ProjectKind;
use crate::globals::{PROGRAM_REPO_URL, PROGRAM_VERSION};
use std::rc::Rc;

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

pub fn readme(
    project_name: &str,
    project_kind: &ProjectKind,
    project_authors: Option<Vec<Rc<str>>>,
) -> String {
    let mut begin = vec![
        String::from("<div align=\"center\">"),
        String::new(),
        format!("{}", project_name),
        format!("{}", "=".repeat(project_name.len())),
        String::new(),
        format!("[![built in](https://img.shields.io/badge/built_in-kojamp_{}-blue?)](https://github.com/nasccped/kojamp)", PROGRAM_VERSION),
        format!(
            "[![project kind](https://img.shields.io/badge/project_kind-{}?)](#)", match project_kind {
                ProjectKind::Java => "java-orange",
                _ => "kotlin-blue",
            }
        ),
        String::new(),
        String::from("</div>"),
        String::new(),
        format!(
            "Welcome to the **{}** project{}",
            project_name,
            if project_authors.is_some() {
                " by:\n"
            } else {
                "!"
            }
        ),
    ];

    let authors = if let Some(authors) = project_authors {
        authors
            .iter()
            .map(|aut| format!("- {}", aut))
            .collect::<Vec<_>>()
            .join("\n")
    } else {
        String::from("")
    };
    begin.push(authors);
    begin.join("\n")
}
