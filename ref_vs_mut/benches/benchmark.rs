use criterion::{criterion_group, criterion_main, Criterion};

fn ref_string(s: Option<String>) -> bool{
  if let Some(ref val) = s {
    return val == "test string";
  }
  return false;
}

fn move_string(s: Option<String>) -> bool{
  if let Some(val) = s {
    return val == "test string";
  }
  return false;
}

fn bm1(c: &mut Criterion) {
    c.bench_function("Pass String as reference", |b| b.iter(|| ref_string(Some("test string".to_string()))));
}

fn bm2(c: &mut Criterion) {
    c.bench_function("Pass String with move", |b| b.iter(|| move_string(Some("test string".to_string()))));
}

criterion_group!(benches, bm1, bm2);
criterion_main!(benches);
