use std::time::Duration;

use cute::Runtime;

fn main() {
    let runtime = Runtime::default();

    for id in 0..10 {
        runtime.spawn(async move {
            let duration = Duration::from_millis(rand::random_range(100..1000));

            println!("[worker {:02}] sleeping for {:?}...", id, duration);

            cute::sleep(duration).await;

            println!("[worker {:02}] done after {:?}!", id, duration);
        });
    }

    runtime.run();
}
