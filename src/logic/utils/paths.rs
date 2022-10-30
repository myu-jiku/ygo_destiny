/*
YGO Destiny – A Yu-Gi-Oh! sealed draft simulator written in rust.
Copyright (C) 2022  myujiku

This program is free software: you can redistribute it and/or modify
it under the terms of the GNU General Public License version 3 as
published by the Free Software Foundation.

This program is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
GNU General Public License for more details.

You should have received a copy of the GNU General Public License
along with this program.  If not, see <https://www.gnu.org/licenses/>.
*/

use std::fs;
use std::path::{Path, PathBuf};

use directories::ProjectDirs;
use once_cell::sync::Lazy;

pub struct Paths {
    project_dirs: ProjectDirs,
    ext_folder: &'static str,
    pub img_big_folder: &'static str,
    pub img_small_folder: &'static str,
    pub img_cropped_folder: &'static str,
    pub img_products_folder: &'static str,
}

impl Paths {
    pub fn data_dir(&self) -> &Path {
        return self.project_dirs.data_dir();
    }

    pub fn ext_dir<P: AsRef<Path>>(&self, path: P) -> PathBuf {
        return self.data_dir().join(self.ext_folder).join(path);
    }

    pub fn img_dir(&self) -> PathBuf {
        return self.data_dir().join("img");
    }
}

// Lazy init default paths/directories
pub static PATHS: Lazy<Paths> = Lazy::new(|| {
    let p = Paths {
        project_dirs: ProjectDirs::from("com", "myujiku", "ygo_destiny").unwrap(),
        ext_folder: "ext",
        img_big_folder: "big",
        img_small_folder: "small",
        img_cropped_folder: "cropped",
        img_products_folder: "products",
    };

    // Ensure "$DATA_DIR/ext"
    fs::create_dir_all(p.data_dir().join(p.ext_folder)).unwrap();

    // Ensure image directories
    let img_dir = p.img_dir();
    fs::create_dir_all(img_dir.join(p.img_big_folder)).unwrap();
    fs::create_dir_all(img_dir.join(p.img_small_folder)).unwrap();
    fs::create_dir_all(img_dir.join(p.img_cropped_folder)).unwrap();
    fs::create_dir_all(img_dir.join(p.img_products_folder)).unwrap();

    p
});
