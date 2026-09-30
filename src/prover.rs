use crate::f17::F17;

/// 一轮 Sum-check 的一次元多项式。
///
/// 对于经典 Sum-check：
///
///     g_i(X) = g_i(0) * (1 - X) + g_i(1) * X
///
/// 因此只需要保存 g_i(0) 和 g_i(1)。
#[derive(Debug, Clone, Copy)]
pub struct RoundPolynomial {
    pub evaluations: [F17; 2],
}

impl RoundPolynomial {
    pub fn new(evaluation_at_0: F17, evaluation_at_1: F17) -> Self {
        Self {
            evaluations: [evaluation_at_0, evaluation_at_1],
        }
    }

    pub fn at_zero(&self) -> F17 {
        self.evaluations[0]
    }

    pub fn at_one(&self) -> F17 {
        self.evaluations[1]
    }

    /// 计算 g_i(r)。
    pub fn evaluate(&self, r: F17) -> F17 {
        (F17::one() - r) * self.at_zero()
            + r * self.at_one()
    }
}


/// 经典 Sum-check Prover。
///
/// Prover 持有多线性多项式 g 在 Boolean cube 上的估值表。
///
/// 例如：
///
///     values = [
///         g(0,0),
///         g(0,1),
///         g(1,0),
///         g(1,1),
///     ]
///
/// 第一轮计算：
///
///     g_1(0) = g(0,0) + g(0,1)
///     g_1(1) = g(1,0) + g(1,1)
///
/// 收到 Verifier 的挑战 r_1 后，下一轮把
/// x_1 固定为 r_1：
///
///     g'(x_2)
///       = (1-r_1) g(0,x_2)
///         + r_1 g(1,x_2)
///
/// 因此估值表长度减半。
pub struct SumcheckProver {
    /// 当前尚未应用最近一次 challenge 的估值表。
    values: Vec<F17>,

    /// Verifier 上一轮发送的 challenge。
    ///
    /// 采用 deferred application：
    /// challenge 收到后暂时不修改 values，
    /// 到下一轮开始时再应用。
    outstanding: Option<F17>,
}

impl SumcheckProver {
    /// 创建一个 Prover。
    ///
    /// `values` 是多线性多项式 g 在 Boolean cube 上的估值表。
    pub fn new(values: Vec<F17>) -> Self {
        assert!(
            !values.is_empty() && values.len().is_power_of_two(),
            "函数值数量必须是 2 的幂"
        );

        Self {
            values,
            outstanding: None,
        }
    }

    /// 返回当前估值表。
    ///
    /// 主要用于测试和调试。
    pub fn values(&self) -> &[F17] {
        &self.values
    }

    /// 生成当前这一轮的 Sum-check 多项式。
    ///
    /// 当前 values：
    ///
    ///     [a0, a1, ..., a_{m-1},
    ///      b0, b1, ..., b_{m-1}]
    ///
    /// 则：
    ///
    ///     g_i(0) = sum(a_j)
    ///     g_i(1) = sum(b_j)
    ///
    /// 注意：
    /// 这里不是两两相邻相加，而是前半部分和后半部分。
    pub fn round_polynomial(&mut self) -> RoundPolynomial {
        // 如果上一轮收到了 challenge，
        // 现在才把它应用到当前估值表。
        self.apply_outstanding_challenge();

        let half = self.values.len() / 2;

        let evaluation_at_0 = self.values[..half]
            .iter()
            .copied()
            .fold(F17::zero(), |acc, value| acc + value);

        let evaluation_at_1 = self.values[half..]
            .iter()
            .copied()
            .fold(F17::zero(), |acc, value| acc + value);

        RoundPolynomial::new(
            evaluation_at_0,
            evaluation_at_1,
        )
    }

    /// 接收 Verifier 产生的随机 challenge。
    ///
    /// Prover 不自己生成 challenge。
    pub fn receive_challenge(&mut self, challenge: F17) {
        assert!(
            self.outstanding.is_none(),
            "上一轮 challenge 尚未应用"
        );

        self.outstanding = Some(challenge);
    }

    /// 将上一轮的 challenge 应用到当前估值表。
    ///
    /// 对于：
    ///
    ///     [a_0, ..., a_{m-1},
    ///      b_0, ..., b_{m-1}]
    ///
    /// 和 challenge r：
    ///
    ///     new_i = (1-r) a_i + r b_i
    ///
    /// 应用后表长度减半。
    fn apply_outstanding_challenge(&mut self) {
        let Some(r) = self.outstanding.take() else {
            return;
        };

        let half = self.values.len() / 2;

        let one_minus_r = F17::one() - r;

        let new_values: Vec<F17> = (0..half)
            .map(|i| {
                one_minus_r * self.values[i]
                    + r * self.values[i + half]
            })
            .collect();

        self.values = new_values;
    }

    /// 在最后一轮 challenge 已经收到之后，
    /// 应用这个 challenge，并得到：
    ///
    ///     g(r_1, ..., r_n)
    ///
    /// 这个值是 Sum-check 最终需要检查的多项式值。
    pub fn final_value(&mut self) -> F17 {
        self.apply_outstanding_challenge();

        assert_eq!(
            self.values.len(),
            1,
            "所有 challenge 应用后，估值表应该只剩一个值"
        );

        self.values[0]
    }
}