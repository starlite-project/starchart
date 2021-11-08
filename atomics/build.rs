use autocfg::new;

fn main() {
    let ac = new();
    for root in ["std", "core"] {
        for size in [8, 16, 32, 64, 128] {
            ac.emit_expression_cfg(
                &format!("{}::sync::atomic::AtomicU{}::compare_exchange", root, size),
                &format!("has_atomic_u{}", size),
            );
            ac.emit_expression_cfg(
                &format!("{}::sync::atomic::AtomicI{}::compare_exchange", root, size),
                &format!("has_atomic_i{}", size),
            );
        }
    }
}
