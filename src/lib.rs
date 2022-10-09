use std::{marker::PhantomData, sync::Arc};

use async_trait::async_trait;
use axum_core::extract::{FromRequest, RequestParts};

#[derive(Debug, Clone)]
pub struct SingletonFlake<T> {
    object: T,
}

impl<T> SingletonFlake<T> {
    pub async fn new<C>(constructor: C) -> Self
    where
        C: Constructor<Target = T>,
    {
        SingletonFlake {
            object: constructor.build().await,
        }
    }

    pub fn get(&self) -> &T {
        &self.object
    }
}

#[async_trait]
impl<T, B> FromRequest<B> for SingletonFlake<T>
where
    T: Clone + Send + Sync + 'static,
    B: Send,
{
    type Rejection = ();

    async fn from_request(req: &mut RequestParts<B>) -> Result<Self, Self::Rejection> {
        let object = req.extensions().get::<T>().unwrap();
        Ok(SingletonFlake {
            object: object.clone(),
        })
    }
}

pub struct TransientFlake<C> {
    constructor: C,
}

impl<T, C> TransientFlake<C>
where
    C: Constructor<Target = T>,
{
    pub async fn get(&self) -> T {
        self.constructor.build().await
    }
}

#[async_trait]
impl<C, B> FromRequest<B> for TransientFlake<C>
where
    C: Clone + Send + Sync + 'static,
    B: Send,
{
    type Rejection = ();

    async fn from_request(req: &mut RequestParts<B>) -> Result<Self, Self::Rejection> {
        let constructor = req.extensions().get::<C>().unwrap();
        Ok(TransientFlake {
            constructor: constructor.clone(),
        })
    }
}

pub struct RequestFlake<C, T> {
    object: T,
    phantom: PhantomData<C>,
}

impl<C, T> RequestFlake<C, T>
where
    C: Fn() -> T,
{
    pub fn get(&self) -> &T {
        &self.object
    }

    pub fn get_mut(&mut self) -> &mut T {
        &mut self.object
    }
}

#[async_trait]
pub trait Constructor {
    type Target;
    async fn build(&self) -> Self::Target;
}

#[async_trait]
impl<C, T, B> FromRequest<B> for RequestFlake<C, T>
where
    T: Send,
    C: Constructor<Target = T> + Send + Sync + 'static,
    B: Send,
{
    type Rejection = ();

    async fn from_request(req: &mut RequestParts<B>) -> Result<Self, Self::Rejection> {
        let constructor = req.extensions_mut().get_mut::<C>().unwrap();
        Ok(RequestFlake {
            object: constructor.build().await,
            phantom: PhantomData,
        })
    }
}
