use super::error::ServiceError;
use tokio::sync::{oneshot, OwnedSemaphorePermit};

/// Message sent over buffer
#[derive(Debug)]
pub(crate) struct Message<Request, Fut> {
    pub(crate) request: Request,
    pub(crate) tx: Tx<Fut>,
    pub(super) _permit: OwnedSemaphorePermit,
}

/// Response sender
pub(crate) type Tx<Fut> = oneshot::Sender<Result<Fut, ServiceError>>;

/// Response receiver
pub(crate) type Rx<Fut> = oneshot::Receiver<Result<Fut, ServiceError>>;
