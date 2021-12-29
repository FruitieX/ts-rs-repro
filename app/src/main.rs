use ts_rs::TS;
use types::SharedType;

#[derive(TS)]
#[ts(export)]
struct AppState {
    shared: SharedType,
}

fn main() {}
