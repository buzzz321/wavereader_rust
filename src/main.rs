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
    faces: Vec<i64>,
}

impl Model {
    fn new() -> Model {
        Model {
            vertices: Vec::new(),
            texture_vertices: Vec::new(),
            faces: Vec::new(),
        }
    }
}

fn main() -> std::io::Result<()> {
    let file = File::open("kub.obj")?;

    let buffer = BufReader::new(file);
    let mut vertices: Vec<Vertex> = Vec::new();
    let mut texture_vertices: Vec<Vertex> = Vec::new();

    let mut model = Model::new();
    for line in buffer.lines() {
        let line = line.unwrap();
        match &line[0..2] {
            "v " => {
                //println!("->{}", line);
                let tmp: Vec<&str> = line[2..].split(" ").collect();
                vertices.push(Vertex {
                    x: tmp[0].parse::<f32>().unwrap(),
                    y: tmp[1].parse::<f32>().unwrap(),
                    z: tmp[2].parse::<f32>().unwrap(),
                })
            }
            "vt" => {
                //println!("=>{}", line);
                let tmp: Vec<&str> = line[3..].split(" ").collect();
                texture_vertices.push(Vertex {
                    x: tmp[0].parse::<f32>().unwrap(),
                    y: tmp[1].parse::<f32>().unwrap(),
                    z: if tmp.len() > 2 {
                        tmp[2].parse::<f32>().unwrap()
                    } else {
                        0.0 //looks like we can use 0 here since many have zeros in their models..
                    },
                })
            }
            "f " => {
                //println!("*>{}", line);
                let tmp: Vec<&str> = line[2..].split(" ").collect();
                for face in tmp {
                    let parts: Vec<&str> = face.split('/').collect();

                    //println!("{:?}", parts[0]);
                    let mut index = parts[0].to_string().parse::<usize>().unwrap() - 1;
                    model.vertices.push(vertices[index]);

                    index = parts[1].to_string().parse::<usize>().unwrap() - 1;
                    model.texture_vertices.push(texture_vertices[index]);
                }
            }
            _ => (), //println!("{}", line),
        };
        // println!("{}", line);
    }
    println!("Model = {:?}", model);

    Ok(())
}
