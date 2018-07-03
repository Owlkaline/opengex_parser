extern crate cgmath;

use std::str;
use std::fs::File;
use std::io::{BufRead, BufReader};

use cgmath::Vector4;
use cgmath::Matrix4;
use cgmath::SquareMatrix;

const X: &str = "\"x\"";
const Y: &str = "\"y\"";
const Z: &str = "\"z\"";

const PLAINSTRING: &str = "string";
const STRING: &str = "{string";
const REF: &str = "{ref";
const FLOAT: &str = "{float";
const INDEX: &str = "(index";
const PRIMITIVE: &str = "(primitive";
const ATTRIB: &str = "(attrib";
const POSITION: &str = "\"position\")";
const NORMAL: &str = "\"normal\")";
const DIFFUSE: &str = "\"diffuse\")";
const SPECULAR: &str = "\"specular\")";
const TEXCOORD: &str = "\"texcoord\")";

const FLOAT2: &str = "float[2]";
const FLOAT3: &str = "float[3]";
const FLOAT16: &str = "float[16]";
const UNSIGNED_INT3: &str = "unsigned_int32[3]";

const METRIC: &str = "Metric";
const KEY: &str = "(key";
const EQUALS: &str = "=";

const DISTANCE: &str = "\"distance\")";
const ANGLE: &str = "\"angle\")";
const TIME: &str = "\"time\")";
const UP: &str = "\"up\")";
const FORWARD: &str = "\"forward\")";

const GEOMETRY_NODE: &str = "GeometryNode";
const GEOMETRY_OBJECT: &str = "GeometryObject";
const OPEN_BRACKET: &str = "{";
const CLOSE_BRACKET: &str = "}";

const ZERO_BRACKET: &str = "0)";

const NAME: &str = "Name";
const OBJECT_REF: &str = "ObjectRef";
const MATERIAL_REF: &str = "MaterialRef";
const TRANSFORM: &str = "Transform";

const MESH: &str = "Mesh";
const VERTEXARRAY: &str = "VertexArray";
const INDEXARRAY: &str = "IndexArray";

const MATERIAL: &str = "Material";
const TEXTURE: &str = "Texture";

const COLOUR: &str = "Color";

const ANIMATION: &str = "Animation";
const TRACK: &str = "Track";
const TRACK_KEY: &str = "Key";
const TRACK_TIME: &str = "Time";
const VALUE: &str = "Value";
const CURVE: &str = "(curve";
const KIND: &str = "(kind";
const PLUSCONTROL: &str = "\"+control\")";
const MINUSCONTROL: &str = "\"-control\")";
const TARGET: &str = "(target";
const XPOS: &str = "%xpos)";
const YPOS: &str = "%ypos)";
const ZPOS: &str = "%zpos)";
const XROT: &str = "%xrot)";
const YROT: &str = "%yrot)";
const ZROT: &str = "%zrot)";
const BEZIER: &str = "\"bezier\")";
const LINEAR: &str = "\"linear\")";

const BEGIN: &str = "(begin";
const END: &str = "end";

fn get_raw_float(v: &str) -> Option<f32> {
  let mut result = None;
  
  if let Ok(float) = v.parse::<f32>() {
    result = Some(float);
  }
  result
}

fn get_float(v: Vec<&str>) -> Option<f32> {
  let mut result = None;
  
  let v: Vec<&str> = v[0].split("{").collect();
  let v: Vec<&str> = v[1].split("}").collect();
  if let Ok(float) = v[0].parse::<f32>() {
    result = Some(float);
  }
  
  result
}

fn get_string_value(v: Vec<&str>) -> Option<&str> {
  let v: Vec<&str> = v[0].split("{").collect();
  let v: Vec<&str> = v[1].split("}").collect();
  let result = Some(v[0]);
  
  result
}

fn remove_brackets(v: &str) -> &str {
  let v = v.trim_matches('{');
  let v = v.trim_matches('}');
  let v = v.trim_matches(')');
  let v = v.trim_matches('(');
  let v = v.trim_matches('\"');
  let v = v.trim_matches('/');
  let v = v.trim_matches(',');
  let v = v.trim_matches('}');
  let v = v.trim();
  v
}

#[derive(Clone)]
pub enum Attrib {
  Diffuse,
  Normal,
  Specular,
  Unknown,
}

#[derive(Clone)]
pub enum Curve {
  Linear,
  Bezier,
  Unknown,
}

#[derive(Clone)]
pub enum KeyType {
  Single,
  Double,
  Triple,
  Quad,
  Sixteen,
  PlusControl,
  MinusControl,
}

#[derive(Clone)]
pub enum TargetType {
  Xpos,
  Ypos,
  Zpos,
  Xrot,
  Yrot,
  Zrot,
  Unknown,
}

pub struct Normal {
  pub normal: [f32; 3],
}

pub struct Vertex {
  pub vertex: [f32; 3],
}

pub struct Index {
  index: u32,
}

pub struct UV {
  pub uv: [f32; 2],
}

struct MaterialRef {
  index: i32,
  material_ref: String,
}

impl MaterialRef {
  pub fn new() -> MaterialRef {
    MaterialRef {
      index: 0,
      material_ref: "".to_string(),
    }
  }
}

#[derive(Clone)]
struct Texture {
  texture: String,
  attrib: Attrib,
  
  raw_transform: [f32; 16],
}

impl Texture {
  pub fn new() -> Texture {
    Texture {
      texture: "".to_string(),
      attrib: Attrib::Unknown,
      
      raw_transform: [1.0, 0.0, 0.0, 0.0, 
                      0.0, 1.0, 0.0, 0.0, 
                      0.0, 0.0, 1.0, 0.0, 
                      0.0, 0.0, 0.0, 1.0],
    }
  }
}

#[derive(Clone)]
struct Material {
  name: String,
  material_ref: String,
  textures: Vec<Texture>,
  
  diffuse_colour: [f32; 3],
  specular_colour: [f32; 3],
  specular_power: f32,
}

impl Material {
  pub fn new() -> Material {
    Material {
      name: "".to_string(),
      material_ref: "".to_string(),
      textures: Vec::new(),
      
      diffuse_colour: [0.0, 0.0, 0.0],
      specular_colour: [0.0, 0.0, 0.0],
      specular_power: 0.0,
    }
  }
}

#[derive(Clone)]
struct GeometryObject {
  name: String,
  mesh: String,
  vertex: Vec<[f32; 3]>,
  index: Vec<u32>,
  normal: Vec<[f32; 3]>,
  texcoord: Vec<[f32; 2]>,
}

impl GeometryObject {
  pub fn new() -> GeometryObject {
    GeometryObject {
      name: "".to_string(),
      mesh: "".to_string(),
      vertex: Vec::new(),
      index: Vec::new(),
      normal: Vec::new(),
      texcoord: Vec::new(),
    }
  }
}

struct GeometryNode {
  name: String,
  raw_transform: [f32; 16],
  transform: Matrix4<f32>,
  
  object_ref: String,
  
  materialref: Vec<MaterialRef>, 
  animation: Option<Animation>,
}

impl GeometryNode {
  pub fn new(name: String) -> GeometryNode {
    GeometryNode {
      name: name,
      raw_transform: [1.0, 0.0, 0.0, 0.0, 
                      0.0, 1.0, 0.0, 0.0, 
                      0.0, 0.0, 1.0, 0.0, 
                      0.0, 0.0, 0.0, 1.0],
      transform: Matrix4::identity(),
      object_ref: "".to_string(),
      
      materialref: Vec::new(),
      animation: None,
    }
  }
}

struct Metric {
  distance: f32,
  angle: f32,
  time: f32,
  up: String,
  forward: String,
}

impl Metric {
  pub fn new() -> Metric {
    Metric {
      distance: 1.0,
      angle: 1.0, 
      time: 1.0,
      up: Y.to_string(),
      forward: "".to_string(),
    }
  }
}

#[derive(Clone)]
struct IndexArray {
  index: Vec<u32>,
}

#[derive(Clone)]
struct TexCoordArray {
  texcoord: Vec<[f32; 2]>
}

#[derive(Clone)]
struct NormalArray {
  normal: Vec<[f32; 3]>,
}

#[derive(Clone)]
struct VertexArray {
  attrib: Attrib,
  morph_index: u32,
  vertex: Vec<[f32; 3]>,
}

#[derive(Clone)]
struct Animation {
  begin: f32,
  end: f32,
  tracks: Vec<Track>,
}

#[derive(Clone)]
struct Track {
  target: TargetType,
  time: Time,
  value: Value
}

#[derive(Clone)]
struct Time {
  curve: Curve,
  keys: Vec<Key>, // 1- 3 max
}

#[derive(Clone)]
struct Value {
  curve: Curve,
  keys: Vec<Key>, // 1- 4 max
}

#[derive(Clone)]
struct Key {
  floats: Vec<f32>,
  key_type: KeyType,
}

impl Animation {
  pub fn new() -> Animation {
    Animation {
      begin: 0.0,
      end: 0.0,
      tracks: Vec::new(),
    }
  }
}

impl Track {
  pub fn new() -> Track {
    Track {
      target: TargetType::Unknown,
      time: Time::new(),
      value: Value::new(),
    }
  }
}

impl Time {
  pub fn new() -> Time {
    Time {
      curve: Curve::Unknown,
      keys: Vec::with_capacity(3),
    }
  }
}

impl Value {
  pub fn new() -> Value {
    Value {
      curve: Curve::Unknown,
      keys: Vec::with_capacity(4),
    }
  }
}

impl Key {
  pub fn new() -> Key {
    Key {
      floats: Vec::new(),
      key_type: KeyType::Single,
    }
  }
}

struct InBasicNode {
  num_brackets_open: i32,
  in_use: bool,
}

struct InIndexedNode {
  num_brackets_open: i32,
  in_use: bool,
  position: usize,
}

struct InDoubleIndexedNode {
  num_brackets_open: i32,
  in_use: bool,
  position: usize,
  second_index: usize,
}

impl InBasicNode {
  pub fn new() -> InBasicNode {
    InBasicNode {
      num_brackets_open: -1,
      in_use: false,
    }
  }
}

impl InIndexedNode {
  pub fn new() -> InIndexedNode {
    InIndexedNode {
      num_brackets_open: -1,
      in_use: false,
      position: 0,
    }
  }
}

impl InDoubleIndexedNode {
  pub fn new() -> InDoubleIndexedNode {
    InDoubleIndexedNode {
      num_brackets_open: -1,
      in_use: false,
      position: 0,
      second_index: 0,
    }
  }
}

#[derive(Clone)]
struct FinalModel {
  vertices: VertexArray,
  indices: IndexArray,
  normals: NormalArray,
  texcoords: TexCoordArray,
  material_ref: String,
  animation: Animation,
}

pub struct OpengexPaser {
  metric: Metric,
  models: Vec<FinalModel>,
  materials: Vec<Material>,
}

impl OpengexPaser {
  pub fn new(location: String) -> OpengexPaser {
    let mut metric = Metric::new();
    
    let mut num_nodes: i32 = 0;
    let mut geometry_nodes: Vec<GeometryNode> = Vec::new();
    let mut geometry_objects: Vec<GeometryObject> = Vec::new();
    let mut materials: Vec<Material> = Vec::new();
    
    let mut in_geometrynode = InIndexedNode::new();
    let mut in_transform = InBasicNode::new();
    let mut in_float2 = InBasicNode::new();
    let mut in_float3 = InBasicNode::new();
    let mut in_float16 = InIndexedNode::new();
    let mut in_unsigned_int3 = InBasicNode::new();
    
    let mut in_geometryobject = InIndexedNode::new();
    let mut in_vertexposition = InBasicNode::new();
    let mut in_vertexnormal = InBasicNode::new();
    let mut in_texcoord = InBasicNode::new();
    let mut in_index = InBasicNode::new();
    let mut in_material = InDoubleIndexedNode::new();
    let mut in_texture = InBasicNode::new();
    
    let mut in_animation = InIndexedNode::new();
    let mut in_track = InIndexedNode::new();
    let mut in_time = InIndexedNode::new();
    let mut in_value = InIndexedNode::new();
    
    let mut num_brackets_open = 0;
    
    if let Ok(file) = File::open(location.clone()) {
      let file = BufReader::new(file);
      
      for line in file.lines() {
        let line = line.expect("Unable to read line");
        let line = line.trim();
        let line = line.trim_left();
        let line = line.trim_matches('\t');
        let mut v: Vec<&str> = line.split(" ").collect();
        
        if v[0].contains(FLOAT2) || v[0].contains(FLOAT3) ||  v[0].contains(UNSIGNED_INT3) {
          v[0] = remove_brackets(v[0]);
        }
        
        match v[0] {
          METRIC => {
            if v[1] == KEY && v[2] == EQUALS {
              match v[3] {
                DISTANCE => {
                  if v[4] == FLOAT {
                    if let Some(float) = get_float(vec!(v[5])) {
                      metric.distance = float;
                    }
                  }
                },
                ANGLE => {
                  if v[4] == FLOAT {
                    if let Some(float) = get_float(vec!(v[5])) {
                      metric.angle= float;
                    }
                  }
                },
                TIME => {
                  if v[4] == FLOAT {
                    if let Some(float) = get_float(vec!(v[5])) {
                      metric.time = float;
                    }
                  }
                },
                UP => {
                   if v[4] == STRING {
                     if let Some(dir) = get_string_value(vec!(v[5])) {
                       metric.up = dir.to_string();
                     }
                   }
                },
                FORWARD => {
                  if v[4] == STRING {
                    if let Some(forward) = get_string_value(vec!(v[5])) {
                      metric.forward = forward.to_string();
                    } 
                  }
                },
                _ => {
                  
                }
              }
            }
          },
          GEOMETRY_NODE => {
            in_geometrynode.num_brackets_open = num_brackets_open;
            in_geometrynode.in_use = true;
            in_geometrynode.position = num_nodes as usize;
            
            num_nodes += 1;
            
            geometry_nodes.push(GeometryNode::new(v[1].to_string()));
          },
          NAME => {
            if v[1] == STRING {
              let name = remove_brackets(v[2]);
              if in_geometrynode.in_use {
                geometry_nodes[in_geometrynode.position].name = name.to_string();
              }
              if in_material.in_use {
                materials[in_material.position].name = name.to_string();
              }
            }
          },
          OBJECT_REF => {
            if v[1] == REF {
              let objectref = remove_brackets(v[2]);
              if in_geometrynode.in_use {
                geometry_nodes[in_geometrynode.position].object_ref = objectref.to_string();
              }
            }
          },
          MATERIAL_REF => {
            if v[1] == INDEX {
              if v[2] == EQUALS {
                // material index
                let index_str = remove_brackets(v[3]);
                let mut index = 0;
                if let Ok(int) = index_str.parse::<i32>() {
                   index = int;
                }
                
                if index == 0 { // Material with index = 0 is default
                  if v[4] == REF {
                    if in_geometrynode.in_use {
                      let materialref = remove_brackets(v[5]);
                      let material_index = geometry_nodes[in_geometrynode.position].materialref.len();
                      geometry_nodes[in_geometrynode.position].materialref.push(MaterialRef::new());
                      geometry_nodes[in_geometrynode.position].materialref[material_index].index = index as i32;
                      geometry_nodes[in_geometrynode.position].materialref[material_index].material_ref = materialref.to_string();
                    }
                  }
                }
              }
            }
          },
          TRANSFORM => {
            in_transform.num_brackets_open = num_brackets_open;
            in_transform.in_use = true;
          },
          FLOAT2 => {
            in_float2.num_brackets_open = num_brackets_open;
            in_float2.in_use = true;
          },
          FLOAT3 => {
            in_float3.num_brackets_open = num_brackets_open;
            in_float3.in_use = true;
          },
          FLOAT16 => {
            in_float16.num_brackets_open = num_brackets_open;
            in_float16.in_use = true;
            in_float16.position = 0;
          },
          UNSIGNED_INT3 => {
            in_unsigned_int3.num_brackets_open = num_brackets_open;
            in_unsigned_int3.in_use = true;
          },
          GEOMETRY_OBJECT => {
            let name = remove_brackets(v[1]);
            
            in_geometryobject.num_brackets_open = num_brackets_open;
            in_geometryobject.in_use = true;
            
            let index = geometry_objects.len();
            in_geometryobject.position = index;
            
            geometry_objects.push(GeometryObject::new());
            geometry_objects[index].name = name.to_string();
          },
          MESH => {
            if in_geometryobject.in_use {
              if v[1] == PRIMITIVE {
                if v[2] == EQUALS {
                  let mesh_name = remove_brackets(v[3]);
                  geometry_objects[in_geometryobject.position].mesh = mesh_name.to_string();
                }
              }
            }
          },
          VERTEXARRAY => {
            if in_geometryobject.in_use {
              if v[1] == ATTRIB {
                if v[2] == EQUALS {
                  if v[3] == POSITION {
                    in_vertexposition.num_brackets_open = num_brackets_open;
                    in_vertexposition.in_use = true;
                  } else
                  if v[3] == NORMAL {
                    in_vertexnormal.num_brackets_open = num_brackets_open;
                    in_vertexnormal.in_use = true;
                  } else
                  if v[3] == TEXCOORD {
                    in_texcoord.num_brackets_open = num_brackets_open;
                    in_texcoord.in_use = true;
                  }
                }
              }
            }
          },
          INDEXARRAY => {
            if in_geometryobject.in_use {
              in_index.num_brackets_open = num_brackets_open;
              in_index.in_use = true;
            }
          },
          MATERIAL => {
            in_material.num_brackets_open = num_brackets_open;
            in_material.in_use = true;
            in_material.second_index = 0;
            let materialref = remove_brackets(v[1]);
            let index = materials.len();
            
            materials.push(Material::new());
            materials[index].material_ref = materialref.to_string();
            
            in_material.position = index;
          }
          TEXTURE => {
            if in_material.in_use {
              if v[1]  == ATTRIB {
                if v[2] == EQUALS {
                  in_texture.num_brackets_open = num_brackets_open;
                  in_texture.in_use = true;
                  
                  in_material.second_index = materials[in_material.position].textures.len();
                  
                  materials[in_material.position].textures.push(Texture::new());
                  if v[3] == DIFFUSE {
                    materials[in_material.position].textures[in_material.second_index].attrib = Attrib::Diffuse;
                  }
                  if v[3] == SPECULAR {
                    materials[in_material.position].textures[in_material.second_index].attrib = Attrib::Specular;
                  }
                  if v[3] == NORMAL {
                    materials[in_material.position].textures[in_material.second_index].attrib = Attrib::Normal;
                  }
                }
              }
            }
          },
          PLAINSTRING => {
            if in_material.in_use && in_texture.in_use {
              let texture = remove_brackets(v[1]);
              materials[in_material.position].textures[in_material.second_index].texture = texture.to_string();
            }
          },
          COLOUR => {
            if in_material.in_use {
              if v[1] == ATTRIB {
                if v[2] == EQUALS {
                  if v[3] == DIFFUSE {
                    let float_type = remove_brackets(v[4]);
                    if float_type == FLOAT3 {
                      let mut colour: [f32; 3] = [0.0,0.0,0.0];
                      let mut idx = 0;
                      for i in 0..(v.len()-5) {
                        let value = remove_brackets(v[5 + i]);
                        if let Ok(float) = value.parse::<f32>() {
                          colour[idx] = float;
                          idx += 1;
                          if idx == 3 {
                            let temp_colour = colour;
                            materials[in_material.position].diffuse_colour = temp_colour;
                            idx = 0;
                            colour = [0.0, 0.0, 0.0];
                          }
                        }
                      }
                    }
                  }
                }
              }
            }
          },
          ANIMATION => {
            if in_geometrynode.in_use {
              in_animation.num_brackets_open = num_brackets_open;
              in_animation.in_use = true;
              geometry_nodes[in_geometrynode.position].animation = Some(Animation::new());
              if let Some(ref mut animation) = geometry_nodes[in_geometrynode.position].animation {
                if v.len() > 1 {
                  if v[1] == BEGIN {
                    if v[2] == EQUALS {
                      let begin_time = get_raw_float(remove_brackets(v[3]));
                      if let Some(time) = begin_time {
                        animation.begin = time;
                      }
                      if v[3] == END {
                        if v[4] == EQUALS {
                          let end_time = get_raw_float(remove_brackets(v[5]));
                          if let Some(time) = end_time {
                            animation.end = time;
                          }
                        }
                      }
                    }
                  }
                }
              }
            }
          },
          TRACK => {
            if in_geometrynode.in_use && in_animation.in_use {
              in_track.num_brackets_open = num_brackets_open;
              in_track.in_use = true;
              in_track.position = in_animation.position;
              if let Some(ref mut animation) = geometry_nodes[in_geometrynode.position].animation {
                animation.tracks.push(Track::new());
                let mut target_type = TargetType::Unknown;
                if v[1] == TARGET {
                  if v[2] == EQUALS {
                    match v[3] {
                      XPOS => {
                        target_type = TargetType::Xpos;
                      },
                      YPOS => {
                        target_type = TargetType::Ypos;
                      },
                      ZPOS => {
                        target_type = TargetType::Zpos;
                      },
                      _ => {}
                    }
                    animation.tracks[in_track.position].target = target_type;
                  }
                }
              }
            }
          },
          TRACK_TIME => {
            if in_geometrynode.in_use && in_animation.in_use && in_track.in_use {
              in_time.num_brackets_open = num_brackets_open;
              in_time.in_use = true;
              if let Some(ref mut animation) = geometry_nodes[in_geometrynode.position].animation {
                if v[1] == CURVE {
                  if v[2] == EQUALS {
                    let mut curve_type = Curve::Unknown;
                    if v[3] == LINEAR {
                      curve_type = Curve::Linear;
                    }
                    if v[3] == BEZIER {
                      curve_type = Curve::Bezier;
                    }
                    animation.tracks[in_track.position].time = Time::new();
                    animation.tracks[in_track.position].time.curve = curve_type;
                  }
                }
              }
            }
          },
          VALUE => {
            if in_geometrynode.in_use && in_animation.in_use && in_track.in_use {
              in_value.num_brackets_open = num_brackets_open;
              in_value.in_use = true;
              if let Some(ref mut animation) = geometry_nodes[in_geometrynode.position].animation {
                if v[1] == CURVE {
                  if v[2] == EQUALS {
                    let mut curve_type = Curve::Unknown;
                    if v[3] == LINEAR {
                      curve_type = Curve::Linear;
                    }
                    if v[3] == BEZIER {
                      curve_type = Curve::Bezier;
                    }
                    animation.tracks[in_track.position].value = Value::new();
                    animation.tracks[in_track.position].value.curve = curve_type;
                  }
                }
              }
            }
          },
          TRACK_KEY => {
            if in_geometrynode.in_use && in_animation.in_use && in_track.in_use {
              if let Some(ref mut animation) = geometry_nodes[in_geometrynode.position].animation {
                if in_time.in_use { 
                  let mut key_type = KeyType::Single;
                  let mut offset = 2;
                  match v[1] {
                    FLOAT => {
                      key_type = KeyType::Single;
                    },
                    KIND => {
                      offset = 4;
                      if v[2] == EQUALS {
                        match v[3] {
                          PLUSCONTROL => {
                            key_type = KeyType::PlusControl;
                          },
                          MINUSCONTROL => {
                            key_type = KeyType::MinusControl;
                          },
                          _ => {}
                        }
                      }
                    },
                    _ => {}
                  }
                  animation.tracks[in_track.position].time.keys.push(Key::new());
                  animation.tracks[in_track.position].time.keys[in_time.position].key_type = key_type;
                  for i in offset..v.len() {
                    let value = get_raw_float(remove_brackets(v[i]));
                    if let Some(value) = value {
                      animation.tracks[in_track.position].time.keys[in_time.position].floats.push(value);
                    }
                  }
                  in_time.position += 1;
                }
                if in_value.in_use { 
                  let mut key_type = KeyType::Single;
                  let mut offset = 2;
                  match v[1] {
                    FLOAT => {
                      key_type = KeyType::Single;
                    },
                    KIND => {
                      offset = 4;
                      if v[2] == EQUALS {
                        match v[3] {
                          PLUSCONTROL => {
                            key_type = KeyType::PlusControl;
                          },
                          MINUSCONTROL => {
                            key_type = KeyType::MinusControl;
                          },
                          _ => {}
                        }
                      }
                    },
                    _ => {}
                  }
                  animation.tracks[in_track.position].value.keys.push(Key::new());
                  animation.tracks[in_track.position].value.keys[in_value.position].key_type = key_type;
                  for i in offset..v.len() {
                    let value = get_raw_float(remove_brackets(v[i]));
                    if let Some(value) = value {
                      animation.tracks[in_track.position].value.keys[in_value.position].floats.push(value);
                    }
                  }
                  in_value.position += 1;
                }
              }
            }
          },
          OPEN_BRACKET => {
            num_brackets_open += 1;
          },
          CLOSE_BRACKET => {
            num_brackets_open -= 1;
            if in_geometrynode.in_use {
              if in_geometrynode.num_brackets_open == num_brackets_open {
                in_geometrynode.num_brackets_open = -1;
                in_geometrynode.in_use = false;
                in_animation.position = 0;
              }
            }
            if in_transform.in_use {
              if in_transform.num_brackets_open == num_brackets_open {
                in_transform.num_brackets_open = -1;
                in_transform.in_use = false;
              }
            }
            if in_float2.in_use {
              if in_float2.num_brackets_open == num_brackets_open {
                in_float2.num_brackets_open = -1;
                in_float2.in_use = false;
              }
            }
            if in_float3.in_use {
              if in_float3.num_brackets_open == num_brackets_open {
                in_float3.num_brackets_open = -1;
                in_float3.in_use = false;
              }
            }
            if in_float16.in_use {
              if in_float16.num_brackets_open == num_brackets_open {
                in_float16.num_brackets_open = -1;
                in_float16.in_use = false;
                in_float16.position = 0;
              }
            }
            if in_unsigned_int3.in_use {
              if in_unsigned_int3.num_brackets_open == num_brackets_open {
                in_unsigned_int3.num_brackets_open = -1;
                in_unsigned_int3.in_use = false;
              }
            }
            if in_geometryobject.in_use {
              if in_geometryobject.num_brackets_open == num_brackets_open {
                in_geometryobject.num_brackets_open = -1;
                in_geometryobject.in_use = false;
                in_geometryobject.position = 0;
              }
            }
            if in_vertexposition.in_use {
              if in_vertexposition.num_brackets_open == num_brackets_open {
                in_vertexposition.num_brackets_open = -1;
                in_vertexposition.in_use = false;
              }
            }
            if in_vertexnormal.in_use {
              if in_vertexnormal.num_brackets_open == num_brackets_open {
                in_vertexnormal.num_brackets_open = -1;
                in_vertexnormal.in_use = false;
              }
            }
            if in_texcoord.in_use {
              if in_texcoord.num_brackets_open == num_brackets_open {
                in_texcoord.num_brackets_open = -1;
                in_texcoord.in_use = false;
              }
            }
            if in_index.in_use {
              if in_index.num_brackets_open == num_brackets_open {
                in_index.num_brackets_open = -1;
                in_index.in_use = false;
              }
            }
            if in_material.in_use {
              if in_material.num_brackets_open == num_brackets_open {
                in_material.num_brackets_open = -1;
                in_material.in_use = false;
                in_material.position = 0;
                in_material.second_index = 0;
              }
            }
            if in_texture.in_use {
              if in_texture.num_brackets_open == num_brackets_open {
                in_texture.num_brackets_open = -1;
                in_texture.in_use = false;
              }
            }
            if in_animation.in_use {
              if in_animation.num_brackets_open == num_brackets_open {
                in_animation.num_brackets_open = -1;
                in_animation.in_use = false;
                in_animation.position += 1;
              }
            }
            if in_track.in_use {
              if in_track.num_brackets_open == num_brackets_open {
                in_track.num_brackets_open = -1;
                in_track.in_use = false;
                in_track.position = 0;
              }
            }
            if in_time.in_use {
              if in_time.num_brackets_open == num_brackets_open {
                in_time.num_brackets_open = -1;
                in_time.in_use = false;
                in_time.position = 0;
              }
            }
            if in_value.in_use {
              if in_value.num_brackets_open == num_brackets_open {
                in_value.num_brackets_open = -1;
                in_value.in_use = false;
                in_value.position = 0;
              }
            }
          },
          _ => {
            if v[0].len() > 1 && v[0].contains(char::is_numeric) {
              if in_geometrynode.in_use {
                if in_transform.in_use {
                  if in_float16.in_use {
                    for i in 0..v.len() {
                      let value = remove_brackets(v[i]);
                      if let Ok(float) = value.parse::<f32>() {
                        geometry_nodes[in_geometrynode.position].raw_transform[in_float16.position] = float;
                        in_float16.position += 1;
                      }
                    }
                  }
                }
              }
              if in_geometryobject.in_use {
                if in_vertexposition.in_use {
                  if in_float3.in_use {
                    let mut vtx: [f32; 3] = [0.0,0.0,0.0];
                    let mut idx = 0;
                    for i in 0..v.len() {
                      let value = remove_brackets(v[i]);
                      if let Ok(float) = value.parse::<f32>() {
                        vtx[idx] = float;
                        idx += 1;
                        if idx == 3 {
                          let temp_vtx = vtx;
                          geometry_objects[in_geometryobject.position].vertex.push(temp_vtx);
                          idx = 0;
                          vtx = [0.0, 0.0, 0.0];
                        }
                      }
                    }
                  }
                }
                if in_vertexnormal.in_use {
                  if in_float3.in_use {
                    let mut nrml: [f32; 3] = [0.0,0.0,0.0];
                    let mut idx = 0;
                    for i in 0..v.len() {
                      let value = remove_brackets(v[i]);
                      if let Ok(float) = value.parse::<f32>() {
                        nrml[idx] = float;
                        idx += 1;
                        if idx == 3 {
                          let temp_nrml = nrml;
                          geometry_objects[in_geometryobject.position].normal.push(temp_nrml);
                          idx = 0;
                          nrml = [0.0, 0.0, 0.0];
                        }
                      }
                    }
                  }
                }
                if in_texcoord.in_use {
                  if in_float2.in_use {
                    let mut texcoord: [f32; 2] = [0.0,0.0];
                    let mut idx = 0;
                    for i in 0..v.len() {
                      let value = remove_brackets(v[i]);
                      if let Ok(float) = value.parse::<f32>() {
                        texcoord[idx] = float;
                        idx += 1;
                        if idx == 2 {
                          let temp_texcoord = texcoord;
                          geometry_objects[in_geometryobject.position].texcoord.push(temp_texcoord);
                          idx = 0;
                          texcoord = [0.0, 0.0];
                        }
                      }
                    }
                  }
                }
                if in_index.in_use {
                  if in_unsigned_int3.in_use {
                    for i in 0..v.len() {
                      let value = remove_brackets(v[i]);
                      if let Ok(unsigned) = value.parse::<u32>() {
                        geometry_objects[in_geometryobject.position].index.push(unsigned);
                      }
                    }
                  }
                }
                if in_texture.in_use {
                  if in_transform.in_use {
                    if in_float16.in_use {
                      let mut vtx: [f32; 3] = [0.0,0.0,0.0];
                      let mut idx = 0;
                      for i in 0..v.len() {
                        let value = remove_brackets(v[i]);
                        if let Ok(float) = value.parse::<f32>() {
                          materials[in_material.position].textures[in_material.second_index].raw_transform[in_float16.position] = float;
                          in_float16.position += 1;
                        }
                      }
                    }
                  }
                }
              }
            }
          }
        }
      }
    } else {
      println!("Error: Model file at location {:?} does not exist!", location);
    }
    
    let mut models: Vec<FinalModel> = Vec::with_capacity(geometry_objects.len());
    
    let mut model_index = 0;
    
    for i in 0..geometry_nodes.len() {
      let transform = Matrix4::new(
        geometry_nodes[i].raw_transform[0], geometry_nodes[i].raw_transform[1], geometry_nodes[i].raw_transform[2], geometry_nodes[i].raw_transform[3], 
        geometry_nodes[i].raw_transform[4], geometry_nodes[i].raw_transform[5], geometry_nodes[i].raw_transform[6], geometry_nodes[i].raw_transform[7], 
        geometry_nodes[i].raw_transform[8], geometry_nodes[i].raw_transform[9], geometry_nodes[i].raw_transform[10], geometry_nodes[i].raw_transform[11], 
        geometry_nodes[i].raw_transform[12], geometry_nodes[i].raw_transform[13], geometry_nodes[i].raw_transform[14], geometry_nodes[i].raw_transform[15], 
      );
      
      for j in 0..geometry_objects.len() {
        if geometry_nodes[i].object_ref == geometry_objects[j].name {
          models.push(FinalModel {
            vertices: VertexArray { vertex: Vec::new(), attrib: Attrib::Unknown, morph_index: 0 }, 
            indices: IndexArray { index: Vec::new() }, 
            normals: NormalArray { normal: Vec::new() }, 
            texcoords: TexCoordArray { texcoord: Vec::new() }, //UVArray { uv: Vec::new() },
            material_ref: "".to_string(),
            animation: Animation::new(),
          });
          
          
          let object = geometry_objects[j].clone();
          let vertex = object.vertex;
          let index = object.index;
          let normal = object.normal;
          
          let mut transformed_vertex: Vec<[f32; 3]> = Vec::with_capacity(vertex.len());
          for k in 0..vertex.len() {
            let temp_vtx = Vector4::new(vertex[k][0], vertex[k][1], vertex[k][2], 1.0);
            let mut vtx = transform*temp_vtx;
            if metric.up == Z {
              let value = vtx.y;
              vtx.y = vtx.z;
              vtx.z = value;
            }
            if metric.up == X {
              let value = vtx.y;
              vtx.y = vtx.x;
              vtx.x = value;
            }
            let new_vtx = vtx;
            transformed_vertex.push([new_vtx.x, new_vtx.y, new_vtx.z]);
          }
          
          let mut transformed_normal: Vec<[f32; 3]> = Vec::with_capacity(normal.len());
          for k in 0..normal.len() {
            let temp_nrml = Vector4::new(normal[k][0], normal[k][1], normal[k][2], 1.0);
            let mut nrml = transform*temp_nrml;
            if metric.up == Z {
              let value = nrml.y;
              nrml.y = nrml.z;
              nrml.z = value;
            }
            if metric.up == X {
              let value = nrml.y;
              nrml.y = nrml.x;
              nrml.x = value;
            }
            
            let new_nrml = nrml;
            transformed_normal.push([new_nrml.x, new_nrml.y, new_nrml.z]);
          }
          
          let tex_coord = geometry_objects[j].texcoord.clone();
          
          models[model_index].vertices = VertexArray { vertex: transformed_vertex, attrib: Attrib::Unknown, morph_index: 0 };
          models[model_index].indices = IndexArray { index: index };
          models[model_index].normals = NormalArray { normal: transformed_normal };
          models[model_index].texcoords = TexCoordArray { texcoord: tex_coord };
          if let Some(ref animation) = geometry_nodes[i].animation {
            models[model_index].animation = animation.clone();
          }
          
          for k in 0..geometry_nodes[i].materialref.len() {
            if geometry_nodes[i].materialref[k].index == 0 {
              models[model_index].material_ref = geometry_nodes[i].materialref[k].material_ref.clone();
              break;
            }
          }
          model_index+=1;
          break;
        }
      }
    }
    
    for mut node in geometry_nodes {
      node.materialref.clear();
      node.animation = None;
    }
    
    for mut object in geometry_objects {
      object.vertex.clear();
      object.index.clear();
      object.normal.clear();
      object.texcoord.clear();
    }
    
    OpengexPaser {
      metric: metric,
      models: models,
      materials: materials,
    }
  }
  
  pub fn get_vertex(&self) -> Vec<Vec<[f32; 3]>> {
    let mut vertex: Vec<Vec<[f32; 3]>> = Vec::with_capacity(self.models.len());
    
    for i in 0..self.models.len() {
      let model = self.models[i].clone();
      vertex.push(model.vertices.vertex);
    }
    
    vertex
  }
  
  pub fn get_normal(&self) -> Vec<Vec<[f32; 3]>> {
    let mut normal: Vec<Vec<[f32; 3]>> = Vec::with_capacity(self.models.len());
    
    for i in 0..self.models.len() {
      let model = self.models[i].clone();
      normal.push(model.normals.normal);
    }
    normal
  }
  
  pub fn get_index(&self) -> Vec<Vec<u32>> {
    let mut index: Vec<Vec<u32>> = Vec::with_capacity(self.models.len());
    for i in 0..self.models.len() {
      let model = self.models[i].clone();
      index.push(model.indices.index);
    }
    index
  }
  
  
  pub fn get_texcoords(&self) -> Vec<Vec<[f32; 2]>> {
    let mut texcoords: Vec<Vec<[f32; 2]>> = Vec::with_capacity(self.models.len());
    for i in 0..self.models.len() {
      if self.models[i].texcoords.texcoord.len() > 0 {
        let texcoord = self.models[i].texcoords.texcoord.clone();
        texcoords.push(texcoord);
      }
    }
    texcoords
  }
  
  pub fn get_diffuse_textures(&self) -> Vec<(String, [f32; 3])> {
    let mut textures: Vec<(String, [f32;3])> = Vec::new();
    
    for i in 0..self.materials.len() {
      textures.push(("".to_string(), self.materials[i].diffuse_colour));
      for j in 0..self.materials[i].textures.len() {
        match self.materials[i].textures[j].attrib {
          Attrib::Diffuse => {
            textures[i] = (self.materials[i].textures[j].texture.clone(), self.materials[i].diffuse_colour);
          },
          _ => {},
        }
      }
    }
    
    textures
  }
  
  pub fn get_diffuse_texture(&self, material_ref: String) -> Option<String> {
    let mut texture: Option<String> = None;
    
    for i in 0..self.materials.len() {
      if self.materials[i].material_ref == material_ref {
        for j in 0..self.materials[i].textures.len() {
          match self.materials[i].textures[j].attrib {
            Attrib::Diffuse => {
              texture = Some(self.materials[i].textures[j].texture.clone());
            },
            _ => {},
          }
        }
      }
    }
    
    texture
  }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

