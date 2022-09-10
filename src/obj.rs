use std::io::{BufReader, BufRead};
use std::path::Path;
use std::fs::File;

use crate::mesh::Mesh;
use crate::triangle::Face;
use crate::vector::Vec3;

#[derive(Debug)]
struct FileFormatError;

fn read_vertex(line: &String) -> Result<Vec3, FileFormatError> {
    let mut positions: Vec<f32> = Vec::new();
    
    for position_str in line.split_ascii_whitespace().skip(1) {
        match position_str.parse::<f32>() {
            Ok(pos) => { positions.push(pos); },
            Err(_) => { return Err(FileFormatError) }
        }
    }

    if positions.len() != 3 {
        return Err(FileFormatError);
    }

    Ok(Vec3::new(positions[0], positions[1], positions[2]))
}

fn read_face(line: &String, num_vertices: u16) -> Result<Face, FileFormatError> {
    let mut vertex_indices: Vec<u16> = Vec::new();

    for indices_str in line.split_ascii_whitespace().skip(1) {
        let vertex_index_str = indices_str.split('/').next().unwrap();
        match vertex_index_str.parse::<i32>() {
            Ok(vertex_index) => {
                if vertex_index > 0 {
                    // Vertex indexes start at 0
                    vertex_indices.push((vertex_index as u16) - 1);
                } else {
                    vertex_indices.push(num_vertices - vertex_index as u16 - 1);
                }
            },
            Err(_) => { return Err(FileFormatError); }
        }
    }

    if vertex_indices.len() != 3 {
        return Err(FileFormatError);
    }

    Ok(Face::new(vertex_indices[0], vertex_indices[1], vertex_indices[2]))
}

impl Mesh {
    pub fn from_obj(obj_file_path: &Path) -> Self {
        let obj_file = File::open(obj_file_path).expect("Could not open OBJ file for reading!");

        let mut vertices = Vec::new();
        let mut faces = Vec::new();

        for potential_line in BufReader::new(obj_file).lines() {
            let line = potential_line.expect("Could not read line from OBJ file!");

            if line.starts_with("v ") {
                let vertex = read_vertex(&line).expect(format!("Could not read vertice from line {}!", &line).as_str());
                vertices.push(vertex);
            } else if line.starts_with("f ") {
                let face = read_face(&line, vertices.len() as u16).expect(format!("Could not read face from line {}!", &line).as_str());
                faces.push(face);
            }
        }

        Self { vertices: vertices, faces: faces, rotation: Vec3::new(0.0, 0.0, 0.0) }
    }
}