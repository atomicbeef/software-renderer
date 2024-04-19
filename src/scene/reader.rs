use std::{
    borrow::Cow,
    fs::{self},
    path::Path,
};

use serde::Deserialize;

use crate::{color::Color, mesh::Mesh, texture::Texture, vector::Vec3};

use super::Object;

#[derive(Deserialize)]
struct SceneObject {
    mesh_path: String,
    texture_path: String,
    rotation: Vec3,
    scale: Vec3,
    translation: Vec3,
}

pub enum SceneDeserializeError<'a> {
    ReadError(Cow<'a, str>),
    JsonError(serde_json::Error),
}

impl std::fmt::Display for SceneDeserializeError<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ReadError(path) => {
                write!(f, "could not open scene file for reading at {path}",)
            }
            Self::JsonError(err) => {
                write!(f, "{err}")
            }
        }
    }
}

pub fn read_objects_from_scene(path: &Path) -> Result<Vec<Object>, SceneDeserializeError> {
    let json = fs::read_to_string(path)
        .or_else(|_| Err(SceneDeserializeError::ReadError(path.to_string_lossy())))?;

    let serialized_scene: Vec<SceneObject> =
        serde_json::from_str(&json).map_err(|e| SceneDeserializeError::JsonError(e))?;

    let mut objects = Vec::new();

    for scene_object in serialized_scene.iter() {
        let mesh_path = Path::new("assets/").join(&scene_object.mesh_path);
        let mesh = Mesh::from_obj(
            &mesh_path,
            scene_object.rotation,
            scene_object.scale,
            scene_object.translation,
        );

        let texture_path = Path::new("assets/").join(&scene_object.texture_path);
        let texture = Texture::from_png(&texture_path).unwrap_or_else(|err| {
            eprintln!("Error reading texture: {err}");
            Texture::from_color(1, 1, Color::new(0xFF, 0x00, 0xFF))
        });

        objects.push(Object { mesh, texture });
    }

    Ok(objects)
}
