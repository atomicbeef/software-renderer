use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

use tinyvec::ArrayVec;

use crate::color::Color;
use crate::mesh::Mesh;
use crate::texture::Tex2;
use crate::triangle::Face;
use crate::vector::Vec3;

#[derive(Debug)]
struct FileFormatError;

pub enum FaceParsingOutput {
    One(Face),
    Two((Face, Face)),
}

fn read_vertex(line: &str) -> Result<Vec3, FileFormatError> {
    let mut positions: [f32; 3] = [0.0; 3];

    // A vertex position line should look like this:
    // v <x> <y> <z>
    for (i, position_str) in line
        .split_ascii_whitespace()
        .skip(1)
        .enumerate()
        .take_while(|(i, _)| *i < 3)
    {
        match position_str.parse::<f32>() {
            Ok(pos) => {
                positions[i] = pos;
            }
            Err(_) => return Err(FileFormatError),
        }
    }

    Ok(Vec3::new(positions[0], positions[1], positions[2]))
}

fn read_uv(line: &str) -> Result<Tex2, FileFormatError> {
    let mut uv: [f32; 2] = [0.0; 2];

    // A vertex UV line should look like this:
    // vt <u> <v>
    for (i, uv_str) in line
        .split_ascii_whitespace()
        .skip(1)
        .enumerate()
        .take_while(|(i, _)| *i < 2)
    {
        match uv_str.parse::<f32>() {
            Ok(tex_coord) => uv[i] = tex_coord,
            Err(_) => return Err(FileFormatError),
        }
    }

    // Flip the v coordinate for compatibility with more .OBJ models
    Ok(Tex2::new(uv[0], 1.0 - uv[1]))
}

fn read_faces(
    line: &str,
    num_vertices: u16,
    num_vertex_uvs: u16,
) -> Result<FaceParsingOutput, FileFormatError> {
    let mut vertex_indices: ArrayVec<[u16; 4]> = ArrayVec::new();
    let mut vertex_uvs: ArrayVec<[u16; 4]> = ArrayVec::new();

    // A face line should look like this:
    // f <vertex index>/<uv index> <vertex index>/<uv index> <vertex index>/<uv index> [vertex index]/[uv_index]
    for (_, indices_str) in line
        .split_ascii_whitespace()
        .skip(1)
        .enumerate()
        .take_while(|(i, _)| *i < 4)
    {
        let mut vertex_index_str = indices_str.split('/');

        // Vertex positions
        match vertex_index_str.next().unwrap().parse::<i32>() {
            Ok(vertex_index) => {
                if vertex_index > 0 {
                    // Indices start at 1 for OBJ files
                    vertex_indices.push((vertex_index as u16) - 1);
                } else {
                    vertex_indices.push(num_vertices - vertex_index as u16 - 1);
                }
            }
            Err(_) => {
                return Err(FileFormatError);
            }
        }

        // Vertex UVs
        if let Some(uv_index_str) = vertex_index_str.next() {
            match uv_index_str.parse::<i32>() {
                Ok(uv_index) => {
                    if uv_index > 0 {
                        vertex_uvs.push((uv_index as u16) - 1);
                    } else {
                        vertex_uvs.push(num_vertex_uvs - uv_index as u16 - 1);
                    }
                }
                Err(_) => {
                    return Err(FileFormatError);
                }
            }
        }
    }

    if vertex_indices.len() == 3 {
        return Ok(FaceParsingOutput::One(Face::new(
            vertex_indices[0],
            vertex_indices[1],
            vertex_indices[2],
            vertex_uvs[0],
            vertex_uvs[1],
            vertex_uvs[2],
            Color::new(0, 0xFF, 0xFF),
        )));
    } else if vertex_indices.len() == 4 {
        return Ok(FaceParsingOutput::Two((
            Face::new(
                vertex_indices[0],
                vertex_indices[1],
                vertex_indices[2],
                vertex_uvs[0],
                vertex_uvs[1],
                vertex_uvs[2],
                Color::new(0, 0xFF, 0xFF),
            ),
            Face::new(
                vertex_indices[2],
                vertex_indices[3],
                vertex_indices[0],
                vertex_uvs[2],
                vertex_uvs[3],
                vertex_uvs[0],
                Color::new(0, 0xFF, 0xFF),
            ),
        )));
    } else {
        return Err(FileFormatError);
    }
}

impl Mesh {
    pub fn from_obj(obj_file_path: &Path, rotation: Vec3, scale: Vec3, translation: Vec3) -> Self {
        let obj_file = File::open(obj_file_path).expect("Could not open OBJ file for reading!");

        let mut vertices = Vec::new();
        let mut vertex_uvs: Vec<Tex2> = Vec::new();
        let mut faces = Vec::new();

        for potential_line in BufReader::new(obj_file).lines() {
            let line = potential_line.expect("Could not read line from OBJ file!");

            if line.starts_with("v ") {
                let vertex = read_vertex(&line)
                    .expect(format!("Could not read vertex from line {}!", &line).as_str());
                vertices.push(vertex);
            } else if line.starts_with("vt ") {
                let uv = read_uv(&line)
                    .expect(format!("Could not read vertex UV from line {}!", &line).as_str());
                vertex_uvs.push(uv);
            } else if line.starts_with("f ") {
                let face_parsing_output =
                    read_faces(&line, vertices.len() as u16, vertex_uvs.len() as u16)
                        .expect(format!("Could not read face(s) from line {}!", &line).as_str());

                match face_parsing_output {
                    FaceParsingOutput::One(face) => {
                        faces.push(face);
                    }
                    FaceParsingOutput::Two(parsed_faces) => {
                        faces.push(parsed_faces.0);
                        faces.push(parsed_faces.1);
                    }
                }
            }
        }

        Self::new(vertices, vertex_uvs, faces, rotation, scale, translation)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn assert_face_attributes(
        face: &Face,
        expected_vertex_indices: [u16; 3],
        expected_uv_indices: [u16; 3],
    ) {
        assert_eq!(
            face.a, expected_vertex_indices[0],
            "Face vertex index A is not correct"
        );
        assert_eq!(
            face.b, expected_vertex_indices[1],
            "Face vertex index B is not correct"
        );
        assert_eq!(
            face.c, expected_vertex_indices[2],
            "Face vertex index C is not correct"
        );
        assert_eq!(
            face.a_uv, expected_uv_indices[0],
            "Face vertex UV index A is not correct"
        );
        assert_eq!(
            face.b_uv, expected_uv_indices[1],
            "Face vertex UV index B is not correct"
        );
        assert_eq!(
            face.c_uv, expected_uv_indices[2],
            "Face vertex UV index C is not correct"
        );
    }

    #[test]
    fn model_can_be_read() {
        let model = Mesh::from_obj(
            Path::new("assets/cube.obj"),
            Vec3::default(),
            Vec3::splat(1.0),
            Vec3::default(),
        );

        assert_eq!(
            model.vertices,
            vec![
                Vec3::new(-1.0, -1.0, 1.0),
                Vec3::new(1.0, -1.0, 1.0),
                Vec3::new(-1.0, 1.0, 1.0),
                Vec3::new(1.0, 1.0, 1.0),
                Vec3::new(-1.0, 1.0, -1.0),
                Vec3::new(1.0, 1.0, -1.0),
                Vec3::new(-1.0, -1.0, -1.0),
                Vec3::new(1.0, -1.0, -1.0),
            ],
            "Vertex positions are not correct"
        );

        // UV coordinates are flipped vertically
        assert_eq!(
            model.vertex_uvs,
            vec![
                Tex2::new(1.0, 1.0),
                Tex2::new(0.0, 1.0),
                Tex2::new(1.0, 0.0),
                Tex2::new(0.0, 0.0),
            ],
            "Texture coordinates are not correct"
        );

        assert_eq!(
            model.faces.len(),
            12,
            "Model does not have the correct number of faces"
        );

        // Indices are 0-based in the renderer, but 1-based in the OBJ file
        assert_face_attributes(&model.faces[0], [0, 1, 2], [0, 1, 2]);
        assert_face_attributes(&model.faces[1], [2, 1, 3], [2, 1, 3]);

        assert_face_attributes(&model.faces[2], [2, 3, 4], [0, 1, 2]);
        assert_face_attributes(&model.faces[3], [4, 3, 5], [2, 1, 3]);

        assert_face_attributes(&model.faces[4], [4, 5, 6], [3, 2, 1]);
        assert_face_attributes(&model.faces[5], [6, 5, 7], [1, 2, 0]);

        assert_face_attributes(&model.faces[6], [6, 7, 0], [0, 1, 2]);
        assert_face_attributes(&model.faces[7], [0, 7, 1], [2, 1, 3]);

        assert_face_attributes(&model.faces[8], [1, 7, 3], [0, 1, 2]);
        assert_face_attributes(&model.faces[9], [3, 7, 5], [2, 1, 3]);

        assert_face_attributes(&model.faces[10], [6, 0, 4], [0, 1, 2]);
        assert_face_attributes(&model.faces[11], [4, 0, 2], [2, 1, 3]);
    }
}
