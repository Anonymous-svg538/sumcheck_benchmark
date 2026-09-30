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
    verifier::SumcheckVerifier,
};

fn benchmark_full_sumcheck(c: &mut Criterion) {
    let mut group = c.benchmark_group("sumcheck_full");

    for num_variables in [4usize, 8, 12, 16] {
        let num_values = 1usize << num_variables;

        // 在 benchmark 之外生成测试数据。
        //
        // 因此随机数生成这些 evaluation table 的成本
        // 不会被计入 Sum-check benchmark。
        let mut rng = rng();

        let values: Vec<F17> = (0..num_values)
            .map(|_| F17::new(rng.random_range(0..17)))
            .collect();

        // H = sum g(x), x ∈ {0,1}^n
        //
        // 同样在 benchmark 之外计算。
        let h = values
            .iter()
            .copied()
            .fold(F17::zero(), |acc, value| acc + value);

        group.bench_with_input(
            BenchmarkId::from_parameter(num_variables),
            &values,
            |b, values| {
                b.iter(|| {
                    let mut prover =
                        SumcheckProver::new(values.to_vec());

                    let mut verifier =
                        SumcheckVerifier::new(
                            h,
                            num_variables,
                        );

                    // 完整执行 Sum-check。
                    for _ in 0..num_variables {
                        // Prover 计算：
                        //
                        // g_i(0), g_i(1)
                        let polynomial =
                            prover.round_polynomial();

                        // Verifier：
                        //
                        // 1. 检查 g_i(0) + g_i(1)
                        // 2. 随机生成 challenge
                        // 3. 计算 g_i(r_i)
                        let challenge =
                            verifier.verify_round(polynomial);

                        // Prover 收到 challenge。
                        prover.receive_challenge(challenge);
                    }

                    // Prover 最终得到
                    // g(r_1, ..., r_n)。
                    let prover_value =
                        black_box(prover.final_value());

                    // Verifier 对原始多线性函数进行
                    // oracle query。
                    let oracle_value =
                        black_box(
                            evaluate_multilinear(
                                values,
                                verifier.challenges(),
                            )
                        );

                    // 最终验证。
                    let accepted =
                        verifier.verify_final(oracle_value);

                    black_box(accepted);

                    // 确保 benchmark 确实执行了正确的协议。
                    debug_assert_eq!(
                        prover_value,
                        oracle_value
                    );
                    debug_assert!(accepted);
                });
            },
        );
    }

    group.finish();
}

/// 计算多线性函数：
///
///     g(r_1, ..., r_n)
///
/// values 的排列顺序为：
///
///     g(0,0,...,0)
///     g(0,0,...,1)
///     ...
///     g(1,1,...,1)
///
/// 每固定一个变量，evaluation table 长度减半。
fn evaluate_multilinear(
    values: &[F17],
    challenges: &[F17],
) -> F17 {
    let mut current = values.to_vec();

    for &r in challenges {
        let half = current.len() / 2;

        let one_minus_r =
            F17::one() - r;

        let next: Vec<F17> = (0..half)
            .map(|i| {
                one_minus_r * current[i]
                    + r * current[i + half]
            })
            .collect();

        current = next;
    }

    assert_eq!(
        current.len(),
        1,
        "所有变量固定之后应该只剩一个值"
    );

    current[0]
}

criterion_group!(
    benches,
    benchmark_full_sumcheck
);

criterion_main!(benches);