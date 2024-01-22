#[derive(Debug)]
pub enum InstanceId {
    Connection(String),
}

pub struct HostInstanceState<D> {
    pub id: InstanceId,
    pub data: D,
}

impl<D> HostInstanceState<D> {
    pub fn for_connection(id: impl Into<String>, data: D) -> Self {
        Self {
            id: InstanceId::Connection(id.into()),
            data,
        }
    }
}
