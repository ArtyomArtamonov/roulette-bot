use tokio::macros::support::Future;

pub mod meta;
pub mod roulette;

pub async fn wrap_logging<H, Fut>(handler: H)
where
    H: FnOnce() -> Fut,
    Fut: Future<Output = ()>,
{
    log::trace!("start processing command");
    handler().await;
    log::trace!("end processing command");
}
