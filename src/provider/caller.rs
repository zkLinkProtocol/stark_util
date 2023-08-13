use async_trait::async_trait;
use serde::{de::DeserializeOwned, Serialize};
use starknet::{
    core::{
        types::{BlockId, BlockTag, FunctionCall},
        utils::{get_selector_from_name, NonAsciiNameError},
    },
    providers::{Provider, ProviderError},
};

use crate::{codec::encode::EncodeError, from_slice, primitive::FieldElement, to_field_elements};

#[derive(Debug, thiserror::Error)]
#[error(transparent)]
pub enum Error<E> {
    StarknetError(NonAsciiNameError),
    EncodeError(EncodeError),
    ProviderError(ProviderError<E>),
}

impl<E> From<NonAsciiNameError> for Error<E> {
    fn from(value: NonAsciiNameError) -> Self {
        Error::StarknetError(value)
    }
}

impl<E> From<EncodeError> for Error<E> {
    fn from(value: EncodeError) -> Self {
        Error::EncodeError(value)
    }
}

impl<E> From<ProviderError<E>> for Error<E> {
    fn from(value: ProviderError<E>) -> Self {
        Error::ProviderError(value)
    }
}

#[async_trait]
pub trait Caller {
    async fn caller<I, O>(&self, contract_address: FieldElement, func_name: &str, calldata: I) -> anyhow::Result<O>
        where I: Serialize + Send,
              O: DeserializeOwned + Send;
}

#[async_trait]
impl<T> Caller for T
    where T: Provider + Send + Sync,
          <T as Provider>::Error: 'static
{
    async fn caller<I, O>(&self, contract_address: FieldElement, func_name: &str, calldata: I) -> anyhow::Result<O>
        where I: Serialize + Send,
              O: DeserializeOwned + Send
    {
        let ret = self.call(FunctionCall { contract_address,
                                           entry_point_selector: get_selector_from_name(func_name)?,
                                           calldata: to_field_elements(calldata)? },
                            BlockId::Tag(BlockTag::Pending))
                      .await?;
        Ok(from_slice(ret.as_slice())?)
    }
}
