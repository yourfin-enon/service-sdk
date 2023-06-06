use std::env;

pub fn get_app_info() -> AppInfo {
    let app_name = env::var("APP_NAME");
    let app_name = if let Ok(app_name) = app_name {
        app_name
    } else {
        format!("{}-dev", env!("CARGO_PKG_NAME"))
    };

    let app_version = env::var("APP_VERSION");
    let app_version = if let Ok(app_version) = app_version {
        app_version
    } else {
        format!("{}:dev", env!("CARGO_PKG_VERSION"))
    };

    let app_location = env::var("APP_LOCATION");
    let app_location = if let Ok(app_location) = app_location {
        app_location
    } else {
        "local".to_string()
    };

    AppInfo {
        name: app_name,
        version: app_version,
        location: app_location
    }
}

pub struct AppInfo {
    pub name: String,
    pub version: String,
    pub location: String,
}

impl AppInfo {
    pub fn get_enriched_name(&self) -> String {
        format!("{}-{}:{}", self.name, self.location, self.version)
    }
}
