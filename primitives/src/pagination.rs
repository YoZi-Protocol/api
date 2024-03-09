use derive_builder::Builder;
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, skip_serializing_none};

#[serde_as]
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct PaginationRequest<T>
where
    T: Clone + Default,
{
    #[serde(flatten)]
    query: T,
    size: Option<u64>,
    page: Option<u64>,
}

impl<T: Clone + Default> PaginationRequest<T> {
    pub fn builder() -> PaginationRequest<T> {
        PaginationRequest::<T>::default()
    }

    pub fn query(&self) -> &T {
        &self.query
    }

    pub fn size(&self) -> u64 {
        let size = self.size.unwrap_or(10);

        if size > 100 {
            100
        } else {
            size
        }
    }

    pub fn page(&self) -> u64 {
        self.page.unwrap_or(0)
    }
}

#[serde_as]
#[skip_serializing_none]
#[derive(Builder, Clone, Debug, Default, Serialize, Deserialize)]
#[builder(default, setter(into, strip_option, prefix = "with"))]
pub struct PaginationResponse<T, D>
where
    D: Clone + Default,
    T: Clone + Default,
{
    query: Option<T>,
    size: u64,
    page: u64,
    total: u64,
    #[builder(setter(each(name = "append", into)))]
    data: Vec<D>,
}

impl<D: Clone + Default, T: Clone + Default> PaginationResponse<T, D> {
    pub fn builder() -> PaginationResponseBuilder<T, D> {
        PaginationResponseBuilder::<T, D>::default()
    }
}
