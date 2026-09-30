use std::io;

use sumcheck_benchmark::{
    f17::F17,
    prover::SumcheckProver,
    verifier::SumcheckVerifier,
};

fn main() {
    // =========================
    // 1. 输入多项式估值表
    // =========================

    println!("请输入函数值，用空格分隔：");

    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("读取输入失败");

    let values: Vec<F17> = match input
        .split_whitespace()
        .map(|s| s.parse::<u64>().map(F17::new))
        .collect::<Result<Vec<_>, _>>()
    {
        Ok(values) => values,
        Err(_) => {
            eprintln!("错误：请输入合法的函数值。");
            return;
        }
    };

    // 估值表数量必须是 2 的幂。
    if values.is_empty() || !values.len().is_power_of_two() {
        eprintln!("错误：函数值的数量必须是 2 的幂。");
        return;
    }

    // =========================
    // 2. 显示多项式估值表
    // =========================

    print!("函数值：[");

    for (i, value) in values.iter().enumerate() {
        if i > 0 {
            print!(", ");
        }

        print!("{}", value);
    }

    println!("]");
    println!("函数值数量：{}", values.len());

    // =========================
    // 3. 计算真实 H
    // =========================

    let h = values
        .iter()
        .copied()
        .fold(F17::zero(), |acc, value| acc + value);

    println!("H = {}", h);

    // =========================
    // 4. 初始化 Prover 和 Verifier
    // =========================

    let num_rounds = values.len().ilog2() as usize;

    let mut prover = SumcheckProver::new(values.clone());

    let mut verifier = SumcheckVerifier::new(h, num_rounds);

    println!();
    println!("开始 Sum-check，共 {} 轮。", num_rounds);

    // =========================
    // 5. 执行 Sum-check rounds
    // =========================

    for round in 0..num_rounds {
        println!();
        println!("========== Round {} ==========", round + 1);

        // ---------------------------------
        // Prover → Verifier
        // ---------------------------------

        let polynomial = prover.round_polynomial();

        println!(
            "Prover 发送：g_{}(0) = {}, g_{}(1) = {}",
            round + 1,
            polynomial.at_zero(),
            round + 1,
            polynomial.at_one(),
        );

        // ---------------------------------
        // Verifier 检查，并产生 challenge
        // ---------------------------------

        let challenge = verifier.verify_round(polynomial);

        println!(
            "Verifier 产生 challenge r_{} = {}",
            round + 1,
            challenge
        );

        // ---------------------------------
        // Verifier → Prover
        // ---------------------------------

        prover.receive_challenge(challenge);

        println!(
            "Prover 收到 challenge r_{} = {}",
            round + 1,
            challenge
        );

        // ---------------------------------
        // 显示当前验证状态
        // ---------------------------------

        if verifier.was_rejected() {
            println!("Verifier：目前已经发现错误。");
        } else {
            println!("Verifier：目前尚未发现错误。");
        }
    }

    // =========================
    // 6. Prover 应用最后一个 challenge
    // =========================

    let prover_final_value = prover.final_value();

    println!();
    println!(
        "Prover 最终得到 g(r_1, ..., r_n) = {}",
        prover_final_value
    );

    // =========================
    // 7. Verifier 对原始多项式进行
    //    oracle query
    // =========================

    let challenges = verifier.challenges();

    let oracle_value = evaluate_multilinear(
        &values,
        challenges,
    );

    println!(
        "Verifier oracle query 得到 g(r_1, ..., r_n) = {}",
        oracle_value
    );

    // =========================
    // 8. 最终验证
    // =========================

    let accepted = verifier.verify_final(oracle_value);

    println!();

    if accepted {
        println!("Sum-check: ACCEPT");
    } else {
        println!("Sum-check: REJECT");
    }
}

/// 在任意随机点
/// (r_1, ..., r_n)
/// 上计算原始多线性多项式 g。
///
/// `values` 按照：
///
///     g(0,...,0)
///     g(0,...,1)
///     ...
///     g(1,...,1)
///
/// 的顺序排列。
fn evaluate_multilinear(
    values: &[F17],
    challenges: &[F17],
) -> F17 {
    assert_eq!(
        values.len(),
        1usize << challenges.len(),
        "估值表大小与 challenge 数量不匹配"
    );

    let mut current = values.to_vec();

    for &r in challenges {
        let half = current.len() / 2;

        let one_minus_r = F17::one() - r;

        current = (0..half)
            .map(|i| {
                one_minus_r * current[i]
                    + r * current[i + half]
            })
            .collect();
    }

    assert_eq!(current.len(), 1);

    current[0]
}