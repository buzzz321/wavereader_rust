use std::fs::File;
use std::io::{prelude::*, BufReader};
#[derive(Debug, Copy, Clone)]
struct Vertex {
    x: f32,
    y: f32,
    z: f32,
}
#[derive(Debug)]
struct Model {
    vertices: Vec<Vertex>,
    texture_vertices: Vec<Vertex>,
    normals: Vec<f32>,
    faces: Vec<i64>,
}

impl Model {
    fn new() -> Model {
        Model {
            vertices: Vec::new(),
            texture_vertices: Vec::new(),
            normals: Vec::new(),
            faces: Vec::new(),
        }
    }
}

fn read_obj(filename: &str) -> std::io::Result<Model> {
    let file = File::open(filename)?;

    let buffer = BufReader::new(file);
    let mut no_faces = 0;
    let mut no_normals = 0;

    let mut model = Model::new();
    for line in buffer.lines() {
        let line = line.unwrap();
        match &line[0..2] {
            "v " => {
                //println!("->{}", line);
                let tmp: Vec<&str> = line[2..].split(" ").collect();
                model.vertices.push(Vertex {
                    x: tmp[0].parse::<f32>().unwrap(),
                    y: tmp[1].parse::<f32>().unwrap(),
                    z: tmp[2].parse::<f32>().unwrap(),
                });
            }
            "vt" => {
                //println!("=>{}", line);
                let tmp: Vec<&str> = line[3..].split(" ").collect();
                model.texture_vertices.push(Vertex {
                    x: tmp[0].parse::<f32>().unwrap(),
                    y: tmp[1].parse::<f32>().unwrap(),
                    z: if tmp.len() > 2 {
                        tmp[2].parse::<f32>().unwrap()
                    } else {
                        0.0 //looks like we can use 0 here since many have zeros in their models..
                    },
                })
            }
            "vn" => {
                //println!("=>{}", line);
                let tmp: Vec<&str> = line[3..].split(" ").collect();
                model.normals = Vec::from([
                    tmp[0].parse::<f32>().unwrap(),
                    tmp[1].parse::<f32>().unwrap(),
                    tmp[2].parse::<f32>().unwrap(),
                ]);
                no_normals += 1;
            }
            "f " => {
                //println!("*>{}", line);
                let tmp: Vec<&str> = line[2..].split(" ").collect();
                for face in tmp {
                    let parts: Vec<&str> = face.split('/').collect();

                    //println!("{:?}", parts[0]);
                    let index = parts[0].to_string().parse::<usize>().unwrap() - 1;
                    model.faces.push(index as i64);
                    no_faces += 1;
                }
            }
            _ => (), //println!("{}", line),
        };
    }
    println!("vertices: {} normals: {}", no_faces, no_normals);
    Ok(model)
}

fn main() -> std::io::Result<()> {
    let model = read_obj("cylinder.obj")?;
    println!("Model = {:?}", model);

    for index in model.faces {
        println!("index {}", index + 1);
    }

    Ok(())
}
