use std::path::PathBuf;

use manifest_dir_macros::exist_relative_path;

pub fn get_v1_movies_raw() -> PathBuf {
    exist_relative_path!("tests/assets/v1_v0.20.0_movies.dump").into()
}

pub fn get_v1_movies_with_settings() -> PathBuf {
    exist_relative_path!("tests/assets/v1_v0.20.0_movies_with_settings.dump").into()
}

pub fn get_v1_rubygems_with_settings() -> PathBuf {
    exist_relative_path!("tests/assets/v1_v0.20.0_rubygems_with_settings.dump").into()
}

pub fn get_v2_movies_raw() -> PathBuf {
    exist_relative_path!("tests/assets/v2_v0.21.1_movies.dump").into()
}

pub fn get_v2_movies_with_settings() -> PathBuf {
    exist_relative_path!("tests/assets/v2_v0.21.1_movies_with_settings.dump").into()
}

pub fn get_v2_rubygems_with_settings() -> PathBuf {
    exist_relative_path!("tests/assets/v2_v0.21.1_rubygems_with_settings.dump").into()
}

pub fn get_v3_movies_raw() -> PathBuf {
    exist_relative_path!("tests/assets/v3_v0.24.0_movies.dump").into()
}

pub fn get_v3_movies_with_settings() -> PathBuf {
    exist_relative_path!("tests/assets/v3_v0.24.0_movies_with_settings.dump").into()
}

pub fn get_v3_rubygems_with_settings() -> PathBuf {
    exist_relative_path!("tests/assets/v3_v0.24.0_rubygems_with_settings.dump").into()
}

pub fn get_v4_movies_raw() -> PathBuf {
    exist_relative_path!("tests/assets/v4_v0.25.2_movies.dump").into()
}

pub fn get_v4_movies_with_settings() -> PathBuf {
    exist_relative_path!("tests/assets/v4_v0.25.2_movies_with_settings.dump").into()
}

pub fn get_v4_rubygems_with_settings() -> PathBuf {
    exist_relative_path!("tests/assets/v4_v0.25.2_rubygems_with_settings.dump").into()
}
