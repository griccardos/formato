use std::time::Instant;

use formato::*;
fn main() {
    let start = Instant::now();
    for i in 0..1000000 {
        let a = i as f64 / 100.0;
        let a2 = 100.formato("#,##0.0");

        if i % 99981 == 0 {
            println!("{a} {a2} {}", i.formato("test#,###👍"));
        }
    }
    println!(
        "Done in {:.2}s for {}/s or {}ms per number",
        start.elapsed().as_secs_f32(),
        1000000f32 / start.elapsed().as_secs_f32(),
        start.elapsed().as_secs_f32() / 1000000. * 1000000.
    );
}
