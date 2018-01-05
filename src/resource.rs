// TODO: distinguish between "leaf" and "non-leaf" models?
// (Resource and ResourceGroup?)

pub trait Resource {
    /// The underlying model data type
    type Data;
}

pub trait IndexableResource: Resource {
    type IndexResult;
    fn index(&self) -> Self::IndexResult;
}

pub trait GettableResource: Resource {
    /// The type of parameter data passed in a GET operation.
    type GetParameters;
    fn get(&self, params: Self::GetParameters) -> Self::Data;
}
