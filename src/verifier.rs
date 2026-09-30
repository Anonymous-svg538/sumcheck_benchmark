use rand::RngExt;

use crate::f17::F17;
use crate::prover::RoundPolynomial;

pub struct SumcheckVerifier {
    /// Prover 声称的总和 H。
    h: F17,

    /// Sum-check 一共有多少轮。
    num_rounds: usize,

    /// 当前正在验证第几轮。
    round: usize,

    /// 上一轮多项式在随机点上的取值。
    ///
    /// 第一轮没有上一轮，所以初始为 None。
    previous_evaluation: Option<F17>,

    /// Verifier 是否曾经发现过错误。
    ///
    /// 即使为 true，协议仍然继续执行后续轮次。
    rejected: bool,

    /// Verifier 产生过的随机 challenge。
    challenges: Vec<F17>,
}

impl SumcheckVerifier {
    pub fn new(h: F17, num_rounds: usize) -> Self {
        Self {
            h,
            num_rounds,
            round: 0,
            previous_evaluation: None,
            rejected: false,
            challenges: Vec::with_capacity(num_rounds),
        }
    }

    /// 验证当前轮，并产生这一轮的随机 challenge r_i。
    ///
    /// 即使当前轮验证失败，也不会提前结束协议。
    pub fn verify_round(
        &mut self,
        polynomial: RoundPolynomial,
    ) -> F17 {
        assert!(
            self.round < self.num_rounds,
            "Sum-check 已经完成所有轮次"
        );

        // ==========================================
        // 1. 检查当前轮的 sum-check 条件
        //
        // 第一轮：
        //     g_1(0) + g_1(1) = H
        //
        // 后续轮：
        //     g_i(0) + g_i(1)
        //         = g_{i-1}(r_{i-1})
        // ==========================================

        let round_sum =
            polynomial.at_zero() + polynomial.at_one();

        let expected_sum = match self.previous_evaluation {
            None => self.h,
            Some(value) => value,
        };

        if round_sum != expected_sum {
            self.rejected = true;
        }

        // ==========================================
        // 2. Verifier 随机选择 r_i
        // ==========================================

        let mut rng = rand::rng();

        let challenge =
            F17::new(rng.random_range(0..17));

        self.challenges.push(challenge);

        // ==========================================
        // 3. 计算 g_i(r_i)
        //
        // 下一轮需要检查：
        //
        // g_{i+1}(0) + g_{i+1}(1)
        //     = g_i(r_i)
        // ==========================================

        let evaluation =
            polynomial.evaluate(challenge);

        self.previous_evaluation = Some(evaluation);

        self.round += 1;

        challenge
    }

    /// 是否已经完成所有 Sum-check rounds。
    pub fn is_finished(&self) -> bool {
        self.round == self.num_rounds
    }

    /// 返回 Verifier 产生的所有 challenges。
    pub fn challenges(&self) -> &[F17] {
        &self.challenges
    }

    /// 最后一轮完成之后，验证：
    ///
    ///     g_v(r_v) = g(r_1, ..., r_v)
    ///
    /// 其中 oracle_value 是 Verifier 对原始多项式 g
    /// 进行 oracle query 得到的结果。
    pub fn verify_final(&mut self, oracle_value: F17) -> bool {
        if !self.is_finished() {
            return false;
        }

        // 最后一轮之前已经发现过错误。
        if self.rejected {
            return false;
        }

        // 检查：
        //
        //     g_v(r_v) = g(r_1, ..., r_v)
        match self.previous_evaluation {
            Some(expected_value) => oracle_value == expected_value,
            None => false,
        }
    }

    /// 查看协议过程中是否曾经发生过拒绝条件。
    pub fn was_rejected(&self) -> bool {
        self.rejected
    }
}