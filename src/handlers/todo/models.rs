use serde::{Deserialize, Serialize};

#[derive(Debug,Serialize,Deserialize)]
pub struct Todo{
  pub id:u64,
  pub title:String,
  pub  description:String
}