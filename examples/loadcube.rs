extern crate opengex_parser;

use opengex_parser::OpengexPaser;

fn main() {
  let model = OpengexPaser::new(String::from("./examples/data/cube.ogex"));
  
  let vertices = model.get_vertex();
  let normals = model.get_normal();
  let indices = model.get_index();
  let uvs = model.get_uv();
  
  println!("\nVerticies:");
  for vertex in vertices { 
    print!("{:?}", vertex);
  }
  
  println!("\nNormals:");
  for normal in normals { 
    print!("{:?}", normal);
  }

  println!("\nIndices:");
  for index in indices { 
    print!("{:?}", index);
  }
}
