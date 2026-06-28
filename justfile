build:
  cargo build

run EXAMPLE:
  cargo run --release --example {{EXAMPLE}}

bench BENCH:
  cargo bench --bench {{BENCH}}

expand EXAMPLE:
  cargo expand --example {{EXAMPLE}}
