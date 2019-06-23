use std::result;
use std::collections::HashMap;
//use serde::{Deserialize, Serialize};
use bundle::dsl::DenseTensor;

#[derive(Debug)]
pub enum Error {
  TransformError(String),
  InvalidType(String),
  ColumnAlreadyExists(String),
  NoSuchColumn(String)
}
pub type Result<T> = result::Result<T, Error>;

pub trait Transformer {
  fn transform(&self, frame: &mut LeapFrame) -> Result<()>;
}

/*#[derive(Serialize, Deserialize)]
#[derive(Debug)]
pub enum ColData {
  Bool(Vec<bool>),
  String(Vec<String>),
  Byte(Vec<i8>),
  Short(Vec<i16>),
  Int(Vec<i32>),
  Long(Vec<i64>),
  Float(Vec<f32>),
  Double(Vec<f64>),
  ByteString(Vec<Vec<u8>>),

  BoolVector(Vec<Vec<bool>>),
  StringVector(Vec<Vec<String>>),
  ByteVector(Vec<Vec<i8>>),
  ShortVector(Vec<Vec<i16>>),
  IntVector(Vec<Vec<i32>>),
  LongVector(Vec<Vec<i64>>),
  FloatVector(Vec<Vec<f32>>),
  DoubleVector(Vec<Vec<f64>>),
  ByteStringVector(Vec<Vec<Vec<u8>>>),

  BoolTensor(Vec<DenseTensor<bool>>),
  StringTensor(Vec<DenseTensor<String>>),
  ByteTensor(Vec<DenseTensor<i8>>),
  ShortTensor(Vec<DenseTensor<i16>>),
  IntTensor(Vec<DenseTensor<i32>>),
  LongTensor(Vec<DenseTensor<i64>>),
  FloatTensor(Vec<DenseTensor<f32>>),
  DoubleTensor(Vec<DenseTensor<f64>>),
  ByteStringTensor(Vec<DenseTensor<Vec<u8>>>)
}*/
#[derive(Serialize, Deserialize)]
#[serde(untagged)]
#[derive(Debug)]
pub enum ColElement {
  Bool(bool),
  String(String),
  Byte(i8),
  Short(i16),
  Int(i32),
  Long(i64),
  Float(f32),
  Double(f64),
  ByteString(Vec<u8>),

  BoolVector(Vec<bool>),
  StringVector(Vec<String>),
  ByteVector(Vec<i8>),
  ShortVector(Vec<i16>),
  IntVector(Vec<i32>),
  LongVector(Vec<i64>),
  FloatVector(Vec<f32>),
  DoubleVector(Vec<f64>),
  ByteStringVector(Vec<Vec<u8>>),

  BoolTensor(DenseTensor<bool>),
  StringTensor(DenseTensor<String>),
  ByteTensor(DenseTensor<i8>),
  ShortTensor(DenseTensor<i16>),
  IntTensor(DenseTensor<i32>),
  LongTensor(DenseTensor<i64>),
  FloatTensor(DenseTensor<f32>),
  DoubleTensor(DenseTensor<f64>),
  ByteStringTensor(DenseTensor<Vec<u8>>)
}

/**TODO!! Fix this boilerplate.  Make only a single Enum.  
 * Need to update downstream users (eg, 
 * LinearRegression directly uses ColData)
 */
#[derive(Debug)]
pub enum ColData {
  Bool(Vec<bool>),
  String(Vec<String>),
  Byte(Vec<i8>),
  Short(Vec<i16>),
  Int(Vec<i32>),
  Long(Vec<i64>),
  Float(Vec<f32>),
  Double(Vec<f64>),
  ByteString(Vec<Vec<u8>>),

  BoolVector(Vec<Vec<bool>>),
  StringVector(Vec<Vec<String>>),
  ByteVector(Vec<Vec<i8>>),
  ShortVector(Vec<Vec<i16>>),
  IntVector(Vec<Vec<i32>>),
  LongVector(Vec<Vec<i64>>),
  FloatVector(Vec<Vec<f32>>),
  DoubleVector(Vec<Vec<f64>>),
  ByteStringVector(Vec<Vec<Vec<u8>>>),

  BoolTensor(Vec<DenseTensor<bool>>),
  StringTensor(Vec<DenseTensor<String>>),
  ByteTensor(Vec<DenseTensor<i8>>),
  ShortTensor(Vec<DenseTensor<i16>>),
  IntTensor(Vec<DenseTensor<i32>>),
  LongTensor(Vec<DenseTensor<i64>>),
  FloatTensor(Vec<DenseTensor<f32>>),
  DoubleTensor(Vec<DenseTensor<f64>>),
  ByteStringTensor(Vec<DenseTensor<Vec<u8>>>)
}


/**TODO!! Fix this boilerplate.  Make only a single Enum.  
 * Need to update downstream users (eg, 
 * LinearRegression directly uses ColData)
 * 
 * If we don't fix this boilerplate, need to add more
 * items for Vector and Tensor
 */
impl ColData {
  pub fn append(&mut self, record: ColElement){
    //if let ColData::Bool(c) = self { c }
    //self.push(record);
    match self {
        ColData::Bool(ref mut v) => {
          match record{
            ColElement::Bool(x)=>v.push(x),
            _=>println!("No match")
          }
        },
        ColData::String(ref mut v) => {
          match record{
            ColElement::String(x)=>v.push(x),
            _=>println!("No match")
          }
        },
        ColData::Byte(ref mut v) => {
          match record{
            ColElement::Byte(x)=>v.push(x),
            _=>println!("No match")
          }
        },
        ColData::Short(ref mut v) => {
          match record{
            ColElement::Short(x)=>v.push(x),
            _=>println!("No match")
          }
        },
        ColData::Int(ref mut v) => {
          match record{
            ColElement::Int(x)=>v.push(x),
            _=>println!("No match")
          }
        },
        ColData::Long(ref mut v) => {
          match record{
            ColElement::Long(x)=>v.push(x),
            _=>println!("No match")
          }
        },
        ColData::Float(ref mut v) => {
          match record{
            ColElement::Float(x)=>v.push(x),
            _=>println!("No match")
          }
        },
        ColData::Double(ref mut v) => {
          match record{
            ColElement::Double(x)=>v.push(x),
            _=>println!("No match")
          }
        },
        ColData::ByteString(ref mut v) => {
          match record{
            ColElement::ByteString(x)=>v.push(x),
            _=>println!("No match")
          }
        },
        /*ColElement::String(v) => self.push(v),
        ColElement::Byte(v) => self.push(v),
        ColElement::Short(v) => self.push(v),
        ColElement::Int(v) => self.push(v),
        ColElement::Long(v) => self.push(v),
        ColElement::Float(v) => self.push(v),
        ColElement::Double(v) => self.push(v),
        ColElement::ByteString(v) => self.push(v),

        ColElement::BoolVector(v) => self.push(v),
        ColElement::StringVector(v) => self.push(v),
        ColElement::ByteVector(v) => self.push(v),
        ColElement::ShortVector(v) => self.push(v),
        ColElement::IntVector(v) => self.push(v),
        ColElement::LongVector(v) => self.push(v),
        ColElement::FloatVector(v) => self.push(v),
        ColElement::DoubleVector(v) => self.push(v),
        ColElement::ByteStringVector(v) => self.push(v),

        ColElement::BoolTensor(v) => self.push(v),
        ColElement::StringTensor(v) => self.push(v),
        ColElement::ByteTensor(v) => self.push(v),
        ColElement::ShortTensor(v) => self.push(v),
        ColElement::IntTensor(v) => self.push(v),
        ColElement::LongTensor(v) => self.push(v),
        ColElement::FloatTensor(v) => self.push(v),
        ColElement::DoubleTensor(v) => self.push(v),
        ColElement::ByteStringTensor(v) => self.push(v),*/
        _=>println!("hello")
    }//*/
  }
  pub fn new(record: ColElement)->Self{
    match record {
        ColElement::Bool(v) => ColData::Bool(vec![v]),
        ColElement::String(v) => ColData::String(vec![v]),
        ColElement::Byte(v) => ColData::Byte(vec![v]),
        ColElement::Short(v) => ColData::Short(vec![v]),
        ColElement::Int(v) => ColData::Int(vec![v]),
        ColElement::Long(v) => ColData::Long(vec![v]),
        ColElement::Float(v) => ColData::Float(vec![v]),
        ColElement::Double(v) => ColData::Double(vec![v]),
        ColElement::ByteString(v) => ColData::ByteString(vec![v]),

        ColElement::BoolVector(v) => ColData::BoolVector(vec![v]),
        ColElement::StringVector(v) => ColData::StringVector(vec![v]),
        ColElement::ByteVector(v) => ColData::ByteVector(vec![v]),
        ColElement::ShortVector(v) => ColData::ShortVector(vec![v]),
        ColElement::IntVector(v) => ColData::IntVector(vec![v]),
        ColElement::LongVector(v) => ColData::LongVector(vec![v]),
        ColElement::FloatVector(v) => ColData::FloatVector(vec![v]),
        ColElement::DoubleVector(v) => ColData::DoubleVector(vec![v]),
        ColElement::ByteStringVector(v) => ColData::ByteStringVector(vec![v]),

        ColElement::BoolTensor(v) => ColData::BoolTensor(vec![v]),
        ColElement::StringTensor(v) => ColData::StringTensor(vec![v]),
        ColElement::ByteTensor(v) => ColData::ByteTensor(vec![v]),
        ColElement::ShortTensor(v) => ColData::ShortTensor(vec![v]),
        ColElement::IntTensor(v) => ColData::IntTensor(vec![v]),
        ColElement::LongTensor(v) => ColData::LongTensor(vec![v]),
        ColElement::FloatTensor(v) => ColData::FloatTensor(vec![v]),
        ColElement::DoubleTensor(v) => ColData::DoubleTensor(vec![v]),
        ColElement::ByteStringTensor(v) => ColData::ByteStringTensor(vec![v]),
    }
  }
}

#[derive(Debug)]
pub struct Col {
  name: String,
  data: ColData
}
/**TODO!  Make these private and provide
 * better access functions to manipulate them.
 * Or, see if can deserialize entire leapframe
 * in go (this will require customer 
 * deserializer)
 */
#[derive(Debug)]
pub struct LeapFrame {
  size: usize,
  pub cols: Vec<Col>,
  pub col_indices_by_name: HashMap<String, usize>
}

impl Col {
  pub fn new(name: String, data: ColData) -> Col {
    Col {
      name: name,
      data: data
    }
  }
  pub fn append_record(&mut self, record: ColElement){
    self.data.append(record)
  }

  pub fn from_doubles(name: String, v: Vec<f64>) -> Col {
    Col {
      name: name,
      data: ColData::Double(v)
    }
  }

  pub fn from_double_tensors(name: String, v: Vec<DenseTensor<f64>>) -> Col {
    Col {
      name: name,
      data: ColData::DoubleTensor(v)
    }
  }

  pub fn from_strings(name: String, v: Vec<String>) -> Col {
    Col {
      name: name,
      data: ColData::String(v)
    }
  }

  pub fn from_ints(name: String, v: Vec<i32>) -> Col {
    Col {
      name: name,
      data: ColData::Int(v)
    }
  }

  pub fn from_long_tensors(name: String, v: Vec<DenseTensor<i64>>) -> Col {
    Col {
      name: name,
      data: ColData::LongTensor(v)
    }
  }

  pub fn name(&self) -> &str { &self.name }
  pub fn data(&self) -> &ColData { &self.data }

  pub fn get_doubles(&self) -> Option<&[f64]> {
    match self.data {
      ColData::Double(ref v) => Some(v),
      _ => None
    }
  }
  pub fn try_doubles(&self) -> Result<&[f64]> { Self::option_to_result(self.get_doubles()) }

  pub fn get_ints(&self) -> Option<&[i32]> {
    match self.data {
      ColData::Int(ref v) => Some(v),
      _ => None
    }
  }
  pub fn try_ints(&self) -> Result<&[i32]> { Self::option_to_result(self.get_ints()) }

  pub fn get_double_tensors(&self) -> Option<&[DenseTensor<f64>]> {
    match self.data {
      ColData::DoubleTensor(ref v) => Some(v),
      _ => None
    }
  }
  pub fn try_double_tensors(&self) -> Result<&[DenseTensor<f64>]> { Self::option_to_result(self.get_double_tensors()) }

  pub fn get_strings(&self) -> Option<&[String]> {
    match self.data {
      ColData::String(ref v) => Some(v),
      _ => None
    }
  }
  pub fn try_strings(&self) -> Result<&[String]> { Self::option_to_result(self.get_strings()) }

  fn option_to_result<T>(option: Option<T>) -> Result<T> {
    option.map(|x| Ok(x)).unwrap_or_else(|| Err(Error::InvalidType(String::from(""))))
  }
}

impl LeapFrame {
  pub fn with_size(size: usize) -> LeapFrame {
    LeapFrame {
      size: size,
      cols: Vec::new(),
      col_indices_by_name: HashMap::new()
    }
  }

  pub fn size(&self) -> usize { self.size }
  pub fn cols(&self) -> &[Col] { &self.cols }

  pub fn try_with_doubles(&mut self, name: String, v: Vec<f64>) -> Result<&mut Self> { self.try_with_col(Col::from_doubles(name, v)) }
  pub fn try_with_double_tensors(&mut self, name: String, v: Vec<DenseTensor<f64>>) -> Result<&mut Self> { self.try_with_col(Col::from_double_tensors(name, v)) }
  pub fn try_with_strings(&mut self, name: String, v: Vec<String>) -> Result<&mut Self> { self.try_with_col(Col::from_strings(name, v)) }
  pub fn try_with_ints(&mut self, name: String, v: Vec<i32>) -> Result<&mut Self> { self.try_with_col(Col::from_ints(name, v)) }

  pub fn try_with_col(&mut self, col: Col) -> Result<&mut Self> {
    if self.col_indices_by_name.contains_key(col.name()) {
      Err(Error::ColumnAlreadyExists(String::from(col.name())))
    } else {
      self.col_indices_by_name.insert(col.name().to_string(), self.cols.len());
      self.cols.push(col);

      Ok(self)
    }
  }

  pub fn get_col(&self, name: &str) -> Option<&Col> {
    self.col_indices_by_name.get(name).map(|i| &self.cols[*i])
  }

  pub fn try_col(&self, name: &str) -> Result<&Col> {
    self.get_col(name).map(|x| Ok(x)).unwrap_or_else(|| Err(Error::NoSuchColumn(String::from(name))))
  }

  pub fn try_cols(&self, names: &[String]) -> Result<Vec<&Col>> {
    let mut cols = Vec::with_capacity(names.len());

    for name in names.iter() {
      match self.try_col(name) {
        Ok(col) => cols.push(col),
        Err(err) => return Err(err)
      }
    }

    Ok(cols)
  }

  pub fn get_doubles(&self, name: &str) -> Option<&[f64]> { self.get_col(name).and_then(|c| c.get_doubles()) }
  pub fn try_doubles(&self, name: &str) -> Result<&[f64]> { self.try_col(name).and_then(|c| c.try_doubles()) }

  pub fn get_ints(&self, name: &str) -> Option<&[i32]> { self.get_col(name).and_then(|c| c.get_ints()) }
  pub fn try_ints(&self, name: &str) -> Result<&[i32]> { self.try_col(name).and_then(|c| c.try_ints()) }

  pub fn get_double_tensors(&self, name: &str) -> Option<&[DenseTensor<f64>]> { self.get_col(name).and_then(|c| c.get_double_tensors()) }
  pub fn try_double_tensors(&self, name: &str) -> Result<&[DenseTensor<f64>]> { self.try_col(name).and_then(|c| c.try_double_tensors()) }

  pub fn get_strings(&self, name: &str) -> Option<&[String]> { self.get_col(name).and_then(|c| c.get_strings()) }
  pub fn try_strings(&self, name: &str) -> Result<&[String]> { self.try_col(name).and_then(|c| c.try_strings()) }
}
