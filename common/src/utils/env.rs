pub fn get_env(key: &str) -> String {
    std::env::var(key).unwrap_or_else(|e| {
        dotenvy::var(key).expect(format!("Error while finding env var `{key}`: {:?}", e).as_str())
    })
}

pub fn get_env_optional(key: &str) -> Option<String> {
    dotenvy::var(key).ok()
}

pub fn get_root_dir() -> String {
    let path = project_root::get_project_root();
    if let Ok(path) = path {
        // path.pop();
        path.to_str().unwrap().to_string()
    } else {
        get_env("APP_ROOT")
    }
}
