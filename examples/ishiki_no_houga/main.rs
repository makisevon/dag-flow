use std::collections::HashMap;
use std::time::Instant;

use dag_flow::context::Context;
use dag_flow::engine::Engine;
use futures::StreamExt;
use futures::executor;
use futures::future::OptionFuture;
use futures::stream::FuturesUnordered;

mod tasks;
use tasks::daikichi_yama::DaikichiYama;
use tasks::daikichi_yama::Observatory;
use tasks::kousaka_reina::KousakaReina;
use tasks::kousaka_reina::Trumpet;
use tasks::oumae_kumiko::Euphonium;
use tasks::oumae_kumiko::OumaeKumiko;

fn main() {
    let builder = Engine::builder();
    builder
        .add_task(OumaeKumiko::new())
        .add_task(KousakaReina::new())
        .add_task(DaikichiYama::new());

    let engine = builder.build().unwrap();
    let context = Context::new();

    let now = Instant::now();
    executor::block_on(engine.run(context.clone()));
    assert_eq!(now.elapsed().as_secs(), 1);

    let oumae_kumiko = OumaeKumiko::id();
    let kousaka_reina = KousakaReina::id();
    let daikichi_yama = DaikichiYama::id();

    let data: HashMap<_, _> = executor::block_on(
        vec![&oumae_kumiko, &kousaka_reina, &daikichi_yama]
            .into_iter()
            .map(|id| {
                let data = context.get(id);
                async move { (id, OptionFuture::from(data).await.unwrap_or_default()) }
            })
            .collect::<FuturesUnordered<_>>()
            .collect(),
    );

    assert_eq!(
        data[&oumae_kumiko]
            .clone()
            .and_then(|data| data.downcast::<Euphonium>().ok())
            .map(|euphonium| euphonium.to_string()),
        Some("Euphonium".into())
    );

    assert_eq!(
        data[&kousaka_reina]
            .clone()
            .and_then(|data| data.downcast::<Trumpet>().ok())
            .map(|trumpet| trumpet.to_string()),
        Some("Trumpet".into())
    );

    assert_eq!(
        data[&daikichi_yama]
            .clone()
            .and_then(|data| data.downcast::<Observatory>().ok())
            .map(|observatory| observatory.to_string()),
        Some("Ai wo Mitsuketa Basho".into())
    );
}
