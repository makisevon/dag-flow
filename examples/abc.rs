use std::collections::HashMap;
use std::time::Duration;
use std::time::Instant;

use dag_flow::context::Context;
use dag_flow::engine::Engine;
use dag_flow::task::Input;
use dag_flow::task::Task;
use futures::executor;
use futures::future::OptionFuture;
use futures_timer::Delay;

type Bytes = Vec<u8>;

fn main() {
    let builder = Engine::builder();
    builder.add_task(A).add_task(B).add_task(C);

    let engine = builder.build().unwrap();
    let context = Context::new();

    let now = Instant::now();
    executor::block_on(engine.run(context.clone()));

    let elapsed = now.elapsed().as_secs();
    assert!(elapsed == 2 || elapsed == 3);

    assert_eq!(
        executor::block_on(async {
            OptionFuture::from(context.get(&"C".into()))
                .await
                .unwrap_or_default()
        }),
        Some("C's output".into())
    )
}

struct A;

impl Task<String, Bytes> for A {
    fn id(&self) -> String {
        "A".into()
    }

    async fn run(&self, _: HashMap<String, Input<'_, Bytes>>) -> Option<Bytes> {
        // do something
        Delay::new(Duration::from_secs(1)).await;

        // output
        Some("A's output".into())
    }
}

struct B;

impl Task<String, Bytes> for B {
    fn id(&self) -> String {
        "B".into()
    }

    fn is_auto(&self) -> bool {
        false
    }

    async fn run(&self, _: HashMap<String, Input<'_, Bytes>>) -> Option<Bytes> {
        // do something
        Delay::new(Duration::from_secs(3)).await;

        // output
        Some("B's output".into())
    }
}

struct C;

impl Task<String, Bytes> for C {
    fn id(&self) -> String {
        "C".into()
    }

    fn dependencies(&self) -> Vec<String> {
        vec!["A".into(), "B".into()]
    }

    async fn run(&self, inputs: HashMap<String, Input<'_, Bytes>>) -> Option<Bytes> {
        futures::join!(
            async {
                // do something with `A`'s output
                let _output_a = inputs["A"].clone().await;
                Delay::new(Duration::from_secs(1)).await;
            },
            async {
                // check whether it depends on `B`
                if rand::random_bool(0.5) {
                    // do something with `B`'s output
                    let _output_b = inputs["B"].clone().await;
                }
            }
        );

        // output
        Some("C's output".into())
    }
}
