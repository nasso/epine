#[derive(Debug, Default)]
pub struct Dependency {
    pub path: Option<String>,
    pub git: Option<String>,
    pub target: Option<String>,
    pub name: Option<String>,
}

#[derive(Debug, Default)]
pub struct Target {
    pub name: String,
    pub src: Vec<String>,
    pub include: Vec<String>,
    pub cc: String,
    pub lflags: Vec<String>,
    pub cflags: Vec<String>,
    pub run_before: Vec<String>,
    pub run_after: Vec<String>,
    pub features: Vec<String>,
    pub dependencies: Vec<Dependency>,
}

#[derive(Debug, Default)]
pub struct Manifest {
    pub default_target: String,
    pub dependencies: String,
    pub bin: Vec<Target>,
    pub lib: Vec<Target>,
}
