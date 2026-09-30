use criterion::{
    criterion_group,
    criterion_main,
    BenchmarkId,
    Criterion,
};

use std::hint::black_box;
use rand::{rng, RngExt};

use sumcheck_benchmark::{
    f17::F17,
    prover::SumcheckProver,
};

fn benchmark_prover(c: &mut Criterion) {
    let mut group = c.benchmark_group("sumcheck_prover");

    for num_variables in [4usize, 8, 12, 16] {
        let num_values = 1usize << num_variables;

        // 在 benchmark 之外生成 evaluation table。
        //
        // 因此随机数生成成本不会计入 prover benchmark。
        let mut rng = rng();

        let values: Vec<F17> = (0..num_values)
            .map(|_| F17::new(rng.random_range(0..17)))
            .collect();

        // 在 benchmark 之外生成 verifier challenges。
        //
        // Prover 本身不生成 challenge。
        // 这里预先生成 challenge，避免把随机数生成成本
        // 计入 prover 的性能。
        let challenges: Vec<F17> = (0..num_variables)
            .map(|_| F17::new(rng.random_range(0..17)))
            .collect();

        group.bench_with_input(
            BenchmarkId::from_parameter(num_variables),
            &(&values, &challenges),
            |b, (values, challenges)| {
                b.iter(|| {
                    let mut prover =
                        SumcheckProver::new(values.to_vec());

                    for &challenge in challenges.iter() {
                        // Prover 计算当前轮多项式：
                        //
                        //     g_i(0), g_i(1)
                        let polynomial =
                            prover.round_polynomial();

                        // 确保当前轮多项式的计算不会被优化掉。
                        black_box(polynomial);

                        // 使用 benchmark 外预先生成的 challenge。
                        prover.receive_challenge(challenge);
                    }

                    // 应用最后一个 challenge，
                    // 得到 g(r_1, ..., r_n)。
                    let final_value =
                        prover.final_value();

                    black_box(final_value);
                });
            },
        );
    }

    group.finish();
}

criterion_group!(
    benches,
    benchmark_prover
);

criterion_main!(benches);