use tokio::macros::support::Future;

pub mod meta;
pub mod roulette;

pub async fn wrap_logging<H, Fut>(handler: H)
where
    H: FnOnce() -> Fut,
    Fut: Future<Output = ()>,
{
    log::debug!("start processing command");
    let start = std::time::Instant::now();
    handler().await;
    let elapsed = start.elapsed();
    log::debug!("end processing command; elapsed: {:?}", elapsed);
}
