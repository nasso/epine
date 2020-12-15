use serde::Deserialize;

#[derive(Debug, Default, PartialEq, Deserialize)]
pub struct Dependency {
    pub path: Option<String>,
    pub git: Option<String>,
    pub target: Option<String>,
    pub name: Option<String>,
}

#[derive(Debug, Default, PartialEq, Deserialize)]
pub struct Target {
    pub name: String,
    pub src: Option<Vec<String>>,
    pub include: Option<Vec<String>>,
    pub cc: Option<String>,
    pub lflags: Option<Vec<String>>,
    pub cflags: Option<Vec<String>>,
    pub run_before: Option<Vec<String>>,
    pub run_after: Option<Vec<String>>,
    pub features: Option<Vec<String>>,
    pub dependencies: Option<Vec<Dependency>>,
}

#[derive(Debug, Default, PartialEq, Deserialize)]
pub struct Manifest {
    #[serde(rename = "default-target")]
    pub default_target: String,
    pub dependencies: Option<String>,
    pub bin: Option<Vec<Target>>,
    pub lib: Option<Vec<Target>>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_manifest() {
        let manifest: Manifest = toml::from_str(
            r#"
                default-target = "corewar"
                dependencies = "epine_modules"

                [[bin]]
                name = "corewar"
                src = ["./src/**/*.c"]
                include = ["./include"]
            "#,
        )
        .unwrap();

        assert_eq!(
            manifest,
            Manifest {
                default_target: String::from("corewar"),
                dependencies: Some(String::from("epine_modules")),
                bin: Some(vec![Target {
                    name: String::from("corewar"),
                    src: Some(vec![String::from("./src/**/*.c")]),
                    include: Some(vec![String::from("./include")]),
                    ..Default::default()
                }]),
                ..Default::default()
            }
        );
    }
}
