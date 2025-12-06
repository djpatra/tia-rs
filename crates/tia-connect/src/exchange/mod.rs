

pub trait Connector
where
    Self: Clone + Debug + for<'de> Deserialize<'de> + Serialize,
{

    const ID: E
}
