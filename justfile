cases := '256'

export PROPTEST_CASES := cases

watch FILTER:
  cargo watch --exec 'test -- --nocapture {{FILTER}}'

test:
  cargo test --release -- --nocapture
