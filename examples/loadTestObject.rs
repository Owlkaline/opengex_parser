extern crate opengex_parser;

use opengex_parser::OpengexPaser;

fn main() {
  println!("Loading Static Object");
  let static_model = OpengexPaser::new(String::from("./examples/data/testobject/ObjectStatic.ogex"));
  
  let vertices = static_model.get_vertex();
  let normals = static_model.get_normal();
  let indices = static_model.get_index();
  let texcoords = static_model.get_texcoords();
  
  println!("Loading Rotation Object");
  let rotation_model = OpengexPaser::new(String::from("./examples/data/testobject/ObjectRotationAnimation.ogex"));
  
  let vertices = rotation_model.get_vertex();
  let normals = rotation_model.get_normal();
  let indices = rotation_model.get_index();
  let texcoords = rotation_model.get_texcoords();
  
  println!("Loading Translation Object");
  let translation_model = OpengexPaser::new(String::from("./examples/data/testobject/ObjectTranslationAnimation.ogex"));
  
  let vertices = translation_model.get_vertex();
  let normals = translation_model.get_normal();
  let indices = translation_model.get_index();
  let texcoords = translation_model.get_texcoords();
  
  /*println!("\nVerticies:");
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
  
  println!("\nUVs:");
  for uv in uvs { 
    print!("{:?}", uv);
  }*/
}
