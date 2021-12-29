use ts_rs::TS;

#[derive(TS)]
#[ts(export)]
pub struct SharedType {
  a: f64
}