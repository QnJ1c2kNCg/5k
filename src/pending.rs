/// Creates a future which never resolves
#[derive(Default)]
struct Pending {}

impl Future for Pending {
    type Output = ();

    fn poll(
        self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Self::Output> {
        Poll::Pending
    }
}
