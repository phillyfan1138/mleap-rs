pub mod dsl;
pub mod json;
pub mod ser;
pub mod tform;
pub mod frame;
use std::collections::HashMap;


#[cfg(test)]
mod test {
  use super::*;
  use std::ffi;
  use c;

  #[test]
  fn test_airbnb_serde() {
    let path = "./tmp/model";
    let builder = ser::FileBuilder::try_new(path).unwrap();
    let mut registry = ser::Registry::new();

    registry.insert_op(tform::linear_regression::OP);
    registry.insert_op(tform::string_indexer::OP);
    registry.insert_op(tform::one_hot_encoder::OP);
    registry.insert_op(tform::pipeline::OP);
    registry.insert_op(tform::vector_assembler::OP);
    registry.insert_op(tform::standard_scaler::OP);

    let ctx = ser::Context::new(Box::new(builder), &registry);
    let ctx2 = ctx.try_next("root").unwrap();
    let node = ctx2.read_node().unwrap();

    //note...no need to provide schema.  Schema is inferred by Serde.
    let json_str="[{\"bathrooms\":2.0, \"bedrooms\":3.0, \"security_deposit\":50.0, \"cleaning_fee\":30.0, \"extra_people\":0.0, \"number_of_reviews\":23.0, \"square_feet\":1200.0, \"review_scores_rating\":93.0, \"cancellation_policy\":\"strict\", \"host_is_superhost\":\"1.0\", \"instant_bookable\":\"1.0\", \"room_type\":\"Entire home/apt\", \"state\":\"NY\"}]";

    let mut result: Vec<HashMap<String, frame::ColElement>> = serde_json::from_str(json_str).unwrap();
    let mut frame = frame::LeapFrame::with_size(result.len());
    let mut res_iter=result.iter_mut();
    match res_iter.next() {
      Some(x)=>x.drain().for_each(|(k, v)|{
        frame.try_with_col(frame::Col::new(k, frame::ColData::new(v)));
      }),
      None=>println!("No records")
    };
    //iterate over the rest
    res_iter.for_each(|records|{
      records.drain().for_each(|(k, v)|{
        match frame.col_indices_by_name.get(&k){
          Some(index)=>frame.cols[*index].append_record(v),
          None=>println!("No column named {}", k)
        }
        /*match frame.get_col_mut(&k) {
          Some(&c)=>c.append_record(v),
          None=>println!("No column named {}", k)
        };*/
      });
    });

    /*let mut frame = frame::LeapFrame::with_size(1);
    frame.try_with_doubles(String::from("bathrooms"), vec![2.0]).unwrap();
    frame.try_with_doubles(String::from("bedrooms"), vec![3.0]).unwrap();
    frame.try_with_doubles(String::from("security_deposit"), vec![50.0]).unwrap();
    frame.try_with_doubles(String::from("cleaning_fee"), vec![30.0]).unwrap();
    frame.try_with_doubles(String::from("extra_people"), vec![0.0]).unwrap();
    frame.try_with_doubles(String::from("number_of_reviews"), vec![23.0]).unwrap();
    frame.try_with_doubles(String::from("square_feet"), vec![1200.0]).unwrap();
    frame.try_with_doubles(String::from("review_scores_rating"), vec![93.0]).unwrap();

    frame.try_with_strings(String::from("cancellation_policy"), vec![String::from("strict")]).unwrap();
    frame.try_with_strings(String::from("host_is_superhost"), vec![String::from("1.0")]).unwrap();
    frame.try_with_strings(String::from("instant_bookable"), vec![String::from("1.0")]).unwrap();
    frame.try_with_strings(String::from("room_type"), vec![String::from("Entire home/apt")]).unwrap();
    frame.try_with_strings(String::from("state"), vec![String::from("NY")]).unwrap();*/

    node.transform(&mut frame).unwrap();

    let r = frame.get_doubles("price_prediction").and_then(|x| x.first()).unwrap();

    assert_eq!(*r, 236.76099900182078);
  }
/*
  #[test]
  fn test_airbnb() {
    let path = "./tmp/model";
    let builder = ser::FileBuilder::try_new(path).unwrap();
    let mut registry = ser::Registry::new();

    registry.insert_op(tform::linear_regression::OP);
    registry.insert_op(tform::string_indexer::OP);
    registry.insert_op(tform::one_hot_encoder::OP);
    registry.insert_op(tform::pipeline::OP);
    registry.insert_op(tform::vector_assembler::OP);
    registry.insert_op(tform::standard_scaler::OP);

    let ctx = ser::Context::new(Box::new(builder), &registry);
    let ctx2 = ctx.try_next("root").unwrap();
    let node = ctx2.read_node().unwrap();

    let mut frame = frame::LeapFrame::with_size(1);
    frame.try_with_doubles(String::from("bathrooms"), vec![2.0]).unwrap();
    frame.try_with_doubles(String::from("bedrooms"), vec![3.0]).unwrap();
    frame.try_with_doubles(String::from("security_deposit"), vec![50.0]).unwrap();
    frame.try_with_doubles(String::from("cleaning_fee"), vec![30.0]).unwrap();
    frame.try_with_doubles(String::from("extra_people"), vec![0.0]).unwrap();
    frame.try_with_doubles(String::from("number_of_reviews"), vec![23.0]).unwrap();
    frame.try_with_doubles(String::from("square_feet"), vec![1200.0]).unwrap();
    frame.try_with_doubles(String::from("review_scores_rating"), vec![93.0]).unwrap();

    frame.try_with_strings(String::from("cancellation_policy"), vec![String::from("strict")]).unwrap();
    frame.try_with_strings(String::from("host_is_superhost"), vec![String::from("1.0")]).unwrap();
    frame.try_with_strings(String::from("instant_bookable"), vec![String::from("1.0")]).unwrap();
    frame.try_with_strings(String::from("room_type"), vec![String::from("Entire home/apt")]).unwrap();
    frame.try_with_strings(String::from("state"), vec![String::from("NY")]).unwrap();

    node.transform(&mut frame).unwrap();

    let r = frame.get_doubles("price_prediction").and_then(|x| x.first()).unwrap();

    assert_eq!(*r, 236.76099900182078);
  }

  #[test]
  fn test_airbnb_c() {
    let path = "./tmp/model";
    let c_path = ffi::CString::new(path).unwrap();

    let c_transformer = c::mleap_transformer_load(c_path.as_ptr());
    let c_frame = c::mleap_frame_with_size(1);

    let bathrooms = ffi::CString::new("bathrooms").unwrap();
    let bedrooms = ffi::CString::new("bedrooms").unwrap();
    let security_deposit = ffi::CString::new("security_deposit").unwrap();
    let cleaning_fee = ffi::CString::new("cleaning_fee").unwrap();
    let extra_people = ffi::CString::new("extra_people").unwrap();
    let number_of_reviews = ffi::CString::new("number_of_reviews").unwrap();
    let square_feet = ffi::CString::new("square_feet").unwrap();
    let review_scores_rating = ffi::CString::new("review_scores_rating").unwrap();

    let bathrooms_doubles: Vec<f64> = vec![2.0];
    let bedrooms_doubles: Vec<f64> = vec![3.0];
    let security_deposit_doubles: Vec<f64> = vec![50.0];
    let cleaning_fee_doubles: Vec<f64> = vec![30.0];
    let extra_people_doubles: Vec<f64> = vec![0.0];
    let number_of_reviews_doubles: Vec<f64> = vec![23.0];
    let square_feet_doubles: Vec<f64> = vec![1200.0];
    let review_scores_rating_doubles: Vec<f64> = vec![93.0];

    bathrooms_doubles.as_ptr();
    bedrooms_doubles.as_ptr();
    security_deposit_doubles.as_ptr();
    c::mleap_frame_with_doubles(c_frame, bathrooms.as_ptr(), bathrooms_doubles.as_ptr());
    c::mleap_frame_with_doubles(c_frame, bedrooms.as_ptr(), bedrooms_doubles.as_ptr());
    c::mleap_frame_with_doubles(c_frame, security_deposit.as_ptr(), security_deposit_doubles.as_ptr());
    c::mleap_frame_with_doubles(c_frame, cleaning_fee.as_ptr(), cleaning_fee_doubles.as_ptr());
    c::mleap_frame_with_doubles(c_frame, extra_people.as_ptr(), extra_people_doubles.as_ptr());
    c::mleap_frame_with_doubles(c_frame, number_of_reviews.as_ptr(), number_of_reviews_doubles.as_ptr());
    c::mleap_frame_with_doubles(c_frame, square_feet.as_ptr(), square_feet_doubles.as_ptr());
    c::mleap_frame_with_doubles(c_frame, review_scores_rating.as_ptr(), review_scores_rating_doubles.as_ptr());

    let strict = ffi::CString::new("strict").unwrap();
    let s10 = ffi::CString::new("1.0").unwrap();
    let entire_home = ffi::CString::new("Entire home/apt").unwrap();
    let ny = ffi::CString::new("NY").unwrap();

    let cp = vec![strict.as_ptr()];
    let his = vec![s10.as_ptr()];
    let ib = vec![s10.as_ptr()];
    let rt = vec![entire_home.as_ptr()];
    let states = vec![ny.as_ptr()];

    let cancellation_policy = ffi::CString::new("cancellation_policy").unwrap();
    let host_is_superhost = ffi::CString::new("host_is_superhost").unwrap();
    let instant_bookable = ffi::CString::new("instant_bookable").unwrap();
    let room_type = ffi::CString::new("room_type").unwrap();
    let state = ffi::CString::new("state").unwrap();

    c::mleap_frame_with_strings(c_frame, cancellation_policy.as_ptr(), cp.as_ptr());
    c::mleap_frame_with_strings(c_frame, host_is_superhost.as_ptr(), his.as_ptr());
    c::mleap_frame_with_strings(c_frame, instant_bookable.as_ptr(), ib.as_ptr());
    c::mleap_frame_with_strings(c_frame, room_type.as_ptr(), rt.as_ptr());
    c::mleap_frame_with_strings(c_frame, state.as_ptr(), states.as_ptr());

    c::mleap_transform(c_transformer, c_frame);

    let mut buffer: [f64; 1] = [0.0];
    let price_prediction = ffi::CString::new("price_prediction").unwrap();
    c::mleap_frame_get_doubles(c_frame, price_prediction.as_ptr(), buffer.as_mut_ptr());
    let r = buffer[0];

    assert_eq!(r, 236.76099900182078);

    c::mleap_transformer_free(c_transformer);
    c::mleap_frame_free(c_frame);
  }*/
}
