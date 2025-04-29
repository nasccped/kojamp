use crate::globals::PROGRAM_REPO_URL;

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
