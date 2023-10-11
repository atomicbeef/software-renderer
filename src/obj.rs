use std::io::{BufReader, BufRead};
use std::path::Path;
use std::fs::File;

use crate::color::Color;
use crate::mesh::Mesh;
use crate::texture::Tex2;
use crate::triangle::Face;
use crate::vector::Vec3;

#[derive(Debug)]
struct FileFormatError;

fn read_vertex(line: &str) -> Result<Vec3, FileFormatError> {
    let mut positions: [f32; 3] = [0.0; 3];

    // A vertex position line should look like this:
    // v <x> <y> <z>
    for (i, position_str) in line.split_ascii_whitespace()
        .skip(1)
        .enumerate()
        .take_while(|(i, _)| *i < 3)
    {
        match position_str.parse::<f32>() {
            Ok(pos) => { positions[i] = pos; },
            Err(_) => { return Err(FileFormatError) }
        }
    }

    Ok(Vec3::new(positions[0], positions[1], positions[2]))
}

fn read_uv(line: &str) -> Result<Tex2, FileFormatError> {
    let mut uv: [f32; 2] = [0.0; 2];

    // A vertex UV line should look like this:
    // vt <u> <v>
    for (i, uv_str) in line.split_ascii_whitespace()
        .skip(1)
        .enumerate()
        .take_while(|(i, _)| *i < 2)
    {
        match uv_str.parse::<f32>() {
            Ok(tex_coord) => { uv[i] = tex_coord },
            Err(_) => { return Err(FileFormatError) }
        }
    }

    // Flip the v coordinate for compatibility with more .OBJ models
    Ok(Tex2::new(uv[0], 1.0 - uv[1]))
}

fn read_face(line: &str, num_vertices: u16, num_vertex_uvs: u16) -> Result<Face, FileFormatError> {
    let mut vertex_indices: [u16; 3] = [0; 3];
    let mut vertex_uvs: [u16; 3] = [0; 3];

    // A face line should look like this:
    // f <vertex index>/<uv index> <vertex index>/<uv index> <vertex index>/<uv index>
    for (i, indices_str) in line.split_ascii_whitespace()
        .skip(1)
        .enumerate()
        .take_while(|(i, _)| *i < 3)
    {
        let mut vertex_index_str = indices_str.split('/');
        
        // Vertex positions
        match vertex_index_str.next().unwrap().parse::<i32>() {
            Ok(vertex_index) => {
                if vertex_index > 0 {
                    // Indices start at 1 for OBJ files
                    vertex_indices[i] = (vertex_index as u16) - 1;
                } else {
                    vertex_indices[i] = num_vertices - vertex_index as u16 - 1;
                }
            },
            Err(_) => { return Err(FileFormatError); }
        }

        // Vertex UVs
        if let Some(uv_index_str) = vertex_index_str.next() {
            match uv_index_str.parse::<i32>() {
                Ok(uv_index) => {
                    if uv_index > 0 {
                        vertex_uvs[i] = (uv_index as u16) - 1;
                    } else {
                        vertex_uvs[i] = num_vertex_uvs - uv_index as u16 - 1;
                    }
                },
                Err(_) => { return Err(FileFormatError); }
            }
        }
    }

    if vertex_indices.len() != 3 {
        return Err(FileFormatError);
    }

    Ok(Face::new(
        vertex_indices[0],
        vertex_indices[1],
        vertex_indices[2],
        vertex_uvs[0],
        vertex_uvs[1],
        vertex_uvs[2],
        Color::new(0, 0xFF, 0xFF)
    ))
}

impl Mesh {
    pub fn from_obj(obj_file_path: &Path) -> Self {
        let obj_file = File::open(obj_file_path).expect("Could not open OBJ file for reading!");

        let mut vertices = Vec::new();
        let mut vertex_uvs: Vec<Tex2> = Vec::new();
        let mut faces = Vec::new();

        for potential_line in BufReader::new(obj_file).lines() {
            let line = potential_line.expect("Could not read line from OBJ file!");

            if line.starts_with("v ") {
                let vertex = read_vertex(&line).expect(format!("Could not read vertex from line {}!", &line).as_str());
                vertices.push(vertex);
            } else if line.starts_with("vt ") {
                let uv = read_uv(&line).expect(format!("Could not read vertex UV from line {}!", &line).as_str());
                vertex_uvs.push(uv);
            } else if line.starts_with("f ") {
                let face = read_face(&line, vertices.len() as u16, vertex_uvs.len() as u16)
                    .expect(format!("Could not read face from line {}!", &line).as_str());
                faces.push(face);
            }
        }

        Self {
            vertices,
            vertex_uvs,
            faces,
            rotation: Vec3::default(),
            scale: Vec3::splat(1.0),
            translation: Vec3::default()
        }
    }
}