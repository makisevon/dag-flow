use std::collections::HashMap;
use std::time::Instant;

use dag_flow::context::Context;
use dag_flow::engine::Engine;
use futures::StreamExt;
use futures::executor;
use futures::future::OptionFuture;
use futures::stream::FuturesUnordered;

mod tasks;
use tasks::oumae_kumiko::OumaeKumiko;
use tasks::uji_bashi::UjiBashi;

fn main() {
    let builder = Engine::builder();
    builder
        .add_task(OumaeKumiko::new())
        .add_task(UjiBashi::new());

    let engine = builder.build().unwrap();
    let context = Context::new();

    let now = Instant::now();
    executor::block_on(engine.run(context.clone()));
    assert_eq!(now.elapsed().as_secs(), 1);

    let oumae_kumiko = OumaeKumiko::id();
    let uji_bashi = UjiBashi::id();

    let data: HashMap<_, _> = executor::block_on(
        vec![&oumae_kumiko, &uji_bashi]
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
            .and_then(|data| data.oumae_kumiko().ok())
            .map(|run| run.to_string()),
        Some("Umaku Naritai".into())
    );

    assert_eq!(
        data[&uji_bashi]
            .clone()
            .and_then(|data| data.uji_bashi().ok())
            .map(|cry| cry.to_string()),
        Some("Jigoku no Orphee".into())
    );
}
