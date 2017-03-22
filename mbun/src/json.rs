use serde_json::{Value, Number};
use serde_json::map::Map;
use std::collections::HashMap;
use std::result::Result;
use base64;
use dsl;

pub enum JsonError {
  WriteError(String),
  ReadError(String)
}

pub trait TryFrom<T> where Self: Sized {
  type Err;

  fn try_from(obj: T) -> Result<Self, Self::Err>;
}

impl<'a> From<&'a dsl::Attribute> for Value {
  fn from(value: &'a dsl::Attribute) -> Self {
    match value {
      &dsl::Attribute::Basic(ref basic) => {
        let (t, v) = match basic {
          &dsl::BasicValue::Bool(v) => ("bool", Value::from(v)),
          &dsl::BasicValue::Byte(v) => ("byte", Value::from(v)),
          &dsl::BasicValue::Short(v) => ("short", Value::from(v)),
          &dsl::BasicValue::Int(v) => ("int", Value::from(v)),
          &dsl::BasicValue::Long(v) => ("long", Value::from(v)),
          &dsl::BasicValue::Float(v) => ("float", Value::from(v)),
          &dsl::BasicValue::Double(v) => ("double", Value::from(v)),
          &dsl::BasicValue::ByteString(ref v) => ("byte_string", Value::from(base64::encode(v))),
        };

        let mut map = Map::with_capacity(2);
        map.insert(String::from("type"), Value::from(t));
        map.insert(String::from("value"), v);

        Value::Object(map)
      },
      &dsl::Attribute::Tensor(ref tv) => {
        let dimensions = &tv.dimensions;
        let values = &tv.values;

        let (b, v) = match values {
          &dsl::VectorValue::Bool(ref v) => ("bool", Value::from(v.as_slice())),
          &dsl::VectorValue::Byte(ref v) => ("byte", Value::from(v.as_slice())),
          &dsl::VectorValue::Short(ref v) => ("short", Value::from(v.as_slice())),
          &dsl::VectorValue::Int(ref v) => ("int", Value::from(v.as_slice())),
          &dsl::VectorValue::Long(ref v) => ("long", Value::from(v.as_slice())),
          &dsl::VectorValue::Float(ref v) => ("float", Value::from(v.as_slice())),
          &dsl::VectorValue::Double(ref v) => ("double", Value::from(v.as_slice())),
          &dsl::VectorValue::ByteString(ref v) => {
            let vs: Vec<String> = v.iter().map(|s| base64::encode(s)).collect();
            ("byte_string", Value::from(vs.as_slice()))
          },
          _ => ("double", Value::from(32.0)),
        };

        let mut tmap = Map::with_capacity(2);
        tmap.insert(String::from("dimensions"), Value::from(dimensions.as_slice()));
        tmap.insert(String::from("values"), v);

        let mut map = Map::with_capacity(3);
        map.insert(String::from("type"), Value::from("tensor"));
        map.insert(String::from("base"), Value::from(b));
        map.insert(String::from("value"), Value::Object(tmap));

        Value::Object(map)
      },
      &dsl::Attribute::Array(ref values) => {
        let (b, v) = match values {
          &dsl::VectorValue::Bool(ref v) => ("bool", Value::from(v.as_slice())),
          &dsl::VectorValue::Byte(ref v) => ("byte", Value::from(v.as_slice())),
          &dsl::VectorValue::Short(ref v) => ("short", Value::from(v.as_slice())),
          &dsl::VectorValue::Int(ref v) => ("int", Value::from(v.as_slice())),
          &dsl::VectorValue::Long(ref v) => ("long", Value::from(v.as_slice())),
          &dsl::VectorValue::Float(ref v) => ("float", Value::from(v.as_slice())),
          &dsl::VectorValue::Double(ref v) => ("double", Value::from(v.as_slice())),
          &dsl::VectorValue::ByteString(ref v) => {
            let vs: Vec<String> = v.iter().map(|s| base64::encode(s)).collect();
            ("byte_string", Value::from(vs.as_slice()))
          },
          _ => ("double", Value::from(32.0)),
        };

        let mut map = Map::with_capacity(3);
        map.insert(String::from("type"), Value::from("tensor"));
        map.insert(String::from("base"), Value::from(b));
        map.insert(String::from("value"), v);

        Value::Object(map)
      }
    }
  }
}

fn basic_attribute(basic: dsl::BasicValue) -> dsl::Attribute {
  dsl::Attribute::Basic(basic)
}

impl<'a> TryFrom<&'a Value> for usize {
  type Err = JsonError;

  fn try_from(value: &'a Value) -> Result<Self, Self::Err> {
    value.as_u64().map(|v| Ok(v as usize)).unwrap_or_else(|| Err(JsonError::ReadError("invalid usize".to_string())))
  }
}

impl<'a> TryFrom<&'a Value> for bool {
  type Err = JsonError;

  fn try_from(value: &'a Value) -> Result<Self, Self::Err> {
    value.as_bool().map(|v| Ok(v)).unwrap_or_else(|| Err(JsonError::ReadError("invalid usize".to_string())))
  }
}

impl<'a> TryFrom<&'a Value> for i8 {
  type Err = JsonError;

  fn try_from(value: &'a Value) -> Result<Self, Self::Err> {
    value.as_i64().map(|v| Ok(v as i8)).unwrap_or_else(|| Err(JsonError::ReadError("invalid usize".to_string())))
  }
}

impl<'a> TryFrom<&'a Value> for i16 {
  type Err = JsonError;

  fn try_from(value: &'a Value) -> Result<Self, Self::Err> {
    value.as_i64().map(|v| Ok(v as i16)).unwrap_or_else(|| Err(JsonError::ReadError("invalid usize".to_string())))
  }
}

impl<'a> TryFrom<&'a Value> for i32 {
  type Err = JsonError;

  fn try_from(value: &'a Value) -> Result<Self, Self::Err> {
    value.as_i64().map(|v| Ok(v as i32)).unwrap_or_else(|| Err(JsonError::ReadError("invalid usize".to_string())))
  }
}

impl<'a> TryFrom<&'a Value> for i64 {
  type Err = JsonError;

  fn try_from(value: &'a Value) -> Result<Self, Self::Err> {
    value.as_i64().map(|v| Ok(v as i64)).unwrap_or_else(|| Err(JsonError::ReadError("invalid usize".to_string())))
  }
}

impl<'a> TryFrom<&'a Value> for f32 {
  type Err = JsonError;

  fn try_from(value: &'a Value) -> Result<Self, Self::Err> {
    value.as_f64().map(|v| Ok(v as f32)).unwrap_or_else(|| Err(JsonError::ReadError("invalid usize".to_string())))
  }
}

impl<'a> TryFrom<&'a Value> for f64 {
  type Err = JsonError;

  fn try_from(value: &'a Value) -> Result<Self, Self::Err> {
    value.as_f64().map(|v| Ok(v)).unwrap_or_else(|| Err(JsonError::ReadError("invalid usize".to_string())))
  }
}

impl<'a> TryFrom<&'a Value> for String {
  type Err = JsonError;

  fn try_from(value: &'a Value) -> Result<Self, Self::Err> {
    value.as_str().map(|v| Ok(v.to_string())).unwrap_or_else(|| Err(JsonError::ReadError("invalid usize".to_string())))
  }
}

impl<'a, T: TryFrom<&'a Value, Err=JsonError>> TryFrom<&'a Value> for Vec<T> {
  type Err = JsonError;

  fn try_from(value: &'a Value) -> Result<Self, Self::Err> {
    value.as_array().map(|arr| {
      let mut acc: Vec<T> = Vec::with_capacity(arr.len());

      for v in arr.iter() {
        match T::try_from(v) {
          Ok(c) => acc.push(c),
          Err(err) => return Err(err)
        }
      }

      Ok(acc)
    }).unwrap_or_else(|| Err(JsonError::ReadError("invalid usize".to_string())))
  }
}

impl<'a, T: TryFrom<&'a Value, Err=JsonError>> TryFrom<&'a Value> for HashMap<String, T> {
  type Err = JsonError;

  fn try_from(value: &'a Value) -> Result<Self, Self::Err> {
    value.as_object().map(|map| {
      let mut acc: HashMap<String, T> = HashMap::with_capacity(map.len());

      for (k, v) in map.iter() {
        match T::try_from(v) {
          Ok(c) => acc.insert(k.clone(), c),
          Err(err) => return Err(err)
        };
      }

      Ok(acc)
    }).unwrap_or_else(|| Err(JsonError::ReadError("invalid usize".to_string())))
  }
}

impl<'a> TryFrom<&'a Value> for dsl::Attribute {
  type Err = JsonError;

  fn try_from(value: &'a Value) -> Result<Self, Self::Err> {
    value.as_object().and_then(|map| {
      map.get("type").and_then(|v| v.as_str()).and_then(|tpe| {
        match tpe.as_ref() {
          "tensor" => {
            match (map.get("base"), map.get("value")) {
              (Some(&Value::String(ref base)), Some(tensor)) => {
                tensor.as_object().and_then(|tmap| {
                  match (tmap.get("dimensions"), tmap.get("values")) {
                    (Some(jdims), Some(jvalues)) => {
                      let r = Vec::<usize>::try_from(jdims).and_then(|dims| {
                        (match base.as_ref() {
                          "bool" => Vec::<bool>::try_from(jvalues).map(|v| dsl::VectorValue::Bool(v)),
                          "byte" => Vec::<i8>::try_from(jvalues).map(|v| dsl::VectorValue::Byte(v)),
                          "short" => Vec::<i16>::try_from(jvalues).map(|v| dsl::VectorValue::Short(v)),
                          "int" => Vec::<i32>::try_from(jvalues).map(|v| dsl::VectorValue::Int(v)),
                          "long" => Vec::<i64>::try_from(jvalues).map(|v| dsl::VectorValue::Long(v)),
                          "float" => Vec::<f32>::try_from(jvalues).map(|v| dsl::VectorValue::Float(v)),
                          "double" => Vec::<f64>::try_from(jvalues).map(|v| dsl::VectorValue::Double(v)),
                          _ => Err(JsonError::ReadError("".to_string()))
                        }).map(|values| {
                          dsl::Attribute::Tensor(dsl::TensorValue {
                            dimensions: dims,
                            values: values
                          })
                        })
                      });
                      Some(r)
                    },
                    _ => None
                  }
                })
              },
              _ => None
            }
          },
          "list" => {
            match (map.get("base"), map.get("value")) {
              (Some(&Value::String(ref base)), Some(jvalues)) => {
                let r = (match base.as_ref() {
                  "bool" => Vec::<bool>::try_from(jvalues).map(|v| dsl::VectorValue::Bool(v)),
                  "byte" => Vec::<i8>::try_from(jvalues).map(|v| dsl::VectorValue::Byte(v)),
                  "short" => Vec::<i16>::try_from(jvalues).map(|v| dsl::VectorValue::Short(v)),
                  "int" => Vec::<i32>::try_from(jvalues).map(|v| dsl::VectorValue::Int(v)),
                  "long" => Vec::<i64>::try_from(jvalues).map(|v| dsl::VectorValue::Long(v)),
                  "float" => Vec::<f32>::try_from(jvalues).map(|v| dsl::VectorValue::Float(v)),
                  "double" => Vec::<f64>::try_from(jvalues).map(|v| dsl::VectorValue::Double(v)),
                  _ => Err(JsonError::ReadError("".to_string()))
                }).map(|values| dsl::Attribute::Array(values));

                Some(r)
              },
              _ => None
            }
          },
          "bool" => value.as_bool().map(|v| Ok(basic_attribute(dsl::BasicValue::Bool(v)))),
          "byte" => value.as_i64().map(|v| Ok(basic_attribute(dsl::BasicValue::Byte(v as i8)))),
          "short" => value.as_i64().map(|v| Ok(basic_attribute(dsl::BasicValue::Short(v as i16)))),
          "int" => value.as_i64().map(|v| Ok(basic_attribute(dsl::BasicValue::Int(v as i32)))),
          "long" => value.as_i64().map(|v| Ok(basic_attribute(dsl::BasicValue::Long(v)))),
          "float" => value.as_f64().map(|v| Ok(basic_attribute(dsl::BasicValue::Float(v as f32)))),
          "double" => value.as_f64().map(|v| Ok(basic_attribute(dsl::BasicValue::Double(v)))),
          "byte_string" => {
            value.as_str().map(|v64| {
              base64::decode(v64).
                map(|v| basic_attribute(dsl::BasicValue::ByteString(v))).
                map_err(|_| JsonError::ReadError("Invalid base64 string".to_string()))
            })
          },
          _ => None
        }
      })
    }).unwrap_or_else(|| Err(JsonError::ReadError("".to_string())))
  }
}

impl<'a> From<&'a dsl::Socket> for Value {
  fn from(value: &'a dsl::Socket) -> Self {
    let mut map = Map::with_capacity(2);
    map.insert(String::from("name"), Value::from(value.name()));
    map.insert(String::from("port"), Value::from(value.port()));

    Value::Object(map)
  }
}

impl From<dsl::Socket> for Value {
  fn from(value: dsl::Socket) -> Self {
    Value::from(&value)
  }
}

impl<'a> TryFrom<&'a Value> for dsl::Socket {
  type Err = JsonError;

  fn try_from(value: &'a Value) -> Result<Self, Self::Err> {
    value.as_object().and_then(|map| {
      let m_name = map.get("name").and_then(|x| x.as_str());
      let m_port = map.get("port").and_then(|x| x.as_str());

      match (m_name, m_port) {
        (Some(name), Some(port)) => {
          Some(Ok(dsl::Socket::new(String::from(name), String::from(port))))
        },
        _ => None
      }
    }).unwrap_or_else(|| Err(JsonError::ReadError(String::from("Invalid socket"))))
  }
}

impl<'a> From<&'a dsl::Shape> for Value {
  fn from(value: &'a dsl::Shape) -> Self {
    let inputs = Value::from(value.inputs());
    let outputs = Value::from(value.outputs());
    let mut map = Map::with_capacity(2);
    map.insert(String::from("inputs"), inputs);
    map.insert(String::from("outputs"), outputs);

    Value::Object(map)
  }
}

impl<'a> TryFrom<&'a Value> for dsl::Shape {
  type Err = JsonError;

  fn try_from(value: &'a Value) -> Result<Self, Self::Err> {
    value.as_object().and_then(|map| {
      match (map.get("inputs"), map.get("outputs")) {
        (Some(jinputs), Some(joutputs)) => {
          let tinputs = Vec::<dsl::Socket>::try_from(jinputs);
          let toutputs = Vec::<dsl::Socket>::try_from(joutputs);

          match (tinputs, toutputs) {
            (Ok(inputs), Ok(outputs)) => Some(Ok(dsl::Shape::new(inputs, outputs))),
            _ => None
          }
        },
        _ => None
      }
    }).unwrap_or_else(|| Err(JsonError::ReadError(String::from(""))))
  }
}

impl<'a> From<&'a dsl::Model> for Value {
  fn from(value: &'a dsl::Model) -> Self {
    let mut attrs: Map<String, Value> = value.attributes().
      iter().
      map(|(k, v)| (k.clone(), Value::from(v))).
      collect();

    let mut map = Map::with_capacity(2);
    map.insert(String::from("op"), Value::from(value.op()));
    map.insert(String::from("attributes"), Value::from(attrs));

    Value::Object(map)
  }
}

impl<'a> TryFrom<&'a Value> for dsl::Model {
  type Err = JsonError;

  fn try_from(value: &'a Value) -> Result<Self, Self::Err> {
    value.as_object().and_then(|map| {
      let m_op = map.get("op").and_then(|x| x.as_str());
      let m_attrs = map.get("attributes");

      match (m_op, m_attrs) {
        (Some(op), Some(jattrs)) => {
          let r = HashMap::<String, dsl::Attribute>::try_from(jattrs).map(|attrs| {
            dsl::Model::new(op.to_string(), attrs)
          });
          Some(r)
        },
        _ => None
      }
    }).unwrap_or_else(|| Err(JsonError::ReadError(String::from(""))))
  }
}
