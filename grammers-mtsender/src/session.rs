use std::any::Any;

use futures_core::future::BoxFuture;
use grammers_session::Session;
use grammers_session::types::{DcOption, PeerId, PeerInfo, PeerRef, UpdateState, UpdatesState};

use crate::errors::InvocationError;

pub trait ErasedSession: Send + Sync {
    fn home_dc_id(&self) -> Result<i32, InvocationError>;

    fn set_home_dc_id(&self, dc_id: i32) -> BoxFuture<'_, Result<(), InvocationError>>;

    fn dc_option(&self, dc_id: i32) -> Result<Option<DcOption>, InvocationError>;

    fn set_dc_option(&self, dc_option: &DcOption) -> BoxFuture<'_, Result<(), InvocationError>>;

    fn peer(&self, peer: PeerId) -> BoxFuture<'_, Result<Option<PeerInfo>, InvocationError>>;

    fn peer_ref(&self, peer: PeerId) -> BoxFuture<'_, Result<Option<PeerRef>, InvocationError>>;

    fn cache_peer(&self, peer: &PeerInfo) -> BoxFuture<'_, Result<(), InvocationError>>;

    fn updates_state(&self) -> BoxFuture<'_, Result<UpdatesState, InvocationError>>;

    fn set_update_state(&self, update: UpdateState) -> BoxFuture<'_, Result<(), InvocationError>>;
}

impl<T: Session> ErasedSession for T {
    fn home_dc_id(&self) -> Result<i32, InvocationError> {
        self.home_dc_id()
            .map_err(|e| InvocationError::Session(Box::new(e) as Box<dyn Any + Send + Sync>))
    }

    fn set_home_dc_id(&self, dc_id: i32) -> BoxFuture<'_, Result<(), InvocationError>> {
        Box::pin(async move {
            self.set_home_dc_id(dc_id)
                .await
                .map_err(|e| InvocationError::Session(Box::new(e) as Box<dyn Any + Send + Sync>))
        })
    }

    fn dc_option(&self, dc_id: i32) -> Result<Option<DcOption>, InvocationError> {
        self.dc_option(dc_id)
            .map_err(|e| InvocationError::Session(Box::new(e) as Box<dyn Any + Send + Sync>))
    }

    fn set_dc_option(&self, dc_option: &DcOption) -> BoxFuture<'_, Result<(), InvocationError>> {
        let dc_option = dc_option.clone();
        Box::pin(async move {
            self.set_dc_option(&dc_option)
                .await
                .map_err(|e| InvocationError::Session(Box::new(e) as Box<dyn Any + Send + Sync>))
        })
    }

    fn peer(&self, peer: PeerId) -> BoxFuture<'_, Result<Option<PeerInfo>, InvocationError>> {
        Box::pin(async move {
            self.peer(peer)
                .await
                .map_err(|e| InvocationError::Session(Box::new(e) as Box<dyn Any + Send + Sync>))
        })
    }

    fn peer_ref(&self, peer: PeerId) -> BoxFuture<'_, Result<Option<PeerRef>, InvocationError>> {
        Box::pin(async move {
            self.peer_ref(peer)
                .await
                .map_err(|e| InvocationError::Session(Box::new(e) as Box<dyn Any + Send + Sync>))
        })
    }

    fn cache_peer(&self, peer: &PeerInfo) -> BoxFuture<'_, Result<(), InvocationError>> {
        let peer = peer.clone();
        Box::pin(async move {
            self.cache_peer(&peer)
                .await
                .map_err(|e| InvocationError::Session(Box::new(e) as Box<dyn Any + Send + Sync>))
        })
    }

    fn updates_state(&self) -> BoxFuture<'_, Result<UpdatesState, InvocationError>> {
        Box::pin(async {
            self.updates_state()
                .await
                .map_err(|e| InvocationError::Session(Box::new(e) as Box<dyn Any + Send + Sync>))
        })
    }

    fn set_update_state(&self, update: UpdateState) -> BoxFuture<'_, Result<(), InvocationError>> {
        Box::pin(async {
            self.set_update_state(update)
                .await
                .map_err(|e| InvocationError::Session(Box::new(e) as Box<dyn Any + Send + Sync>))
        })
    }
}
