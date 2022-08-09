use std::{future::Future, task::Poll};

/// Creates a future which never resolves
#[derive(Default)]
pub struct Pending {}

impl Future for Pending {
    type Output = ();

    fn poll(
        self: std::pin::Pin<&mut Self>,
        _: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Self::Output> {
        Poll::Pending
    }
}
