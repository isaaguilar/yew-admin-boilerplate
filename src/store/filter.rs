use yewdux::prelude::*;

#[derive(Store, PartialEq, Default, Debug, Clone)]
pub struct FilterStore {
    pub offset: u64,
    pub cluster: String,
    pub resource_name: String,
    pub namespace: String,
}
