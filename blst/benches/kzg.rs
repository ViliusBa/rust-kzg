use blst_rust::types::fft_settings::FsFFTSettings;
use blst_rust::types::fr::FsFr;
use blst_rust::types::g1::FsG1;
use blst_rust::types::g2::FsG2;
use blst_rust::types::kzg_settings::FsKZGSettings;
use blst_rust::types::poly::FsPoly;
use blst_rust::utils::generate_trusted_setup;
use criterion::{criterion_group, criterion_main, Criterion};
use kzg_bench::benches::kzg::{bench_commit_to_poly, bench_compute_proof_single};

fn bench_commit_to_poly_(c: &mut Criterion) {
    bench_commit_to_poly::<FsFr, FsG1, FsG2, FsPoly, FsFFTSettings, FsKZGSettings>(
        c,
        &generate_trusted_setup,
    )
}

fn bench_compute_proof_single_(c: &mut Criterion) {
    bench_compute_proof_single::<FsFr, FsG1, FsG2, FsPoly, FsFFTSettings, FsKZGSettings>(
        c,
        &generate_trusted_setup,
    )
}

criterion_group! {
    name = benches;
    config = Criterion::default().sample_size(10);
    targets = bench_commit_to_poly_, bench_compute_proof_single_
}

criterion_main!(benches);