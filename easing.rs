use super::uwp::{EasingMode, UwpEasing, apply_uwp};

/// 缓动函数枚举。
/// 只包含经过实际使用验证的函数；其余按需添加。
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Easing {
    Linear,
    EaseOutQuad,
    EaseOutCubic,
    EaseOutQuart,
    EaseInCubic,
    EaseInOutCubic,
    /// 正典 cubic-bezier(x1,y1,x2,y2)（出处见 `bezier_y`）。
    Bezier([f32; 4]),
    /// UWP 缓动变体 + 方向。转发 `apply_uwp`，与 `MetroAnim` 同源同值。
    Uwp(UwpEasing, EasingMode),
}

impl Easing {
    pub fn apply(&self, t: f64) -> f64 {
        match self {
            Easing::Linear => t,
            Easing::EaseOutQuad => ease_out_quad(t),
            Easing::EaseOutCubic => ease_out_cubic(t),
            Easing::EaseOutQuart => ease_out_quart(t),
            Easing::EaseInCubic => ease_in_cubic(t),
            Easing::EaseInOutCubic => ease_in_out_cubic(t),
            Easing::Bezier(c) => bezier_y(*c, t),
            Easing::Uwp(v, m) => apply_uwp(t, v, *m),
        }
    }
}

impl Default for Easing {
    fn default() -> Self {
        Easing::EaseOutCubic
    }
}

/// 平方缓出：快进慢出，轻量版。
pub fn ease_out_quad(t: f64) -> f64 {
    t * (2.0 - t)
}

/// 立方缓出：标准交互缓动，绝大多数场景首选。
pub fn ease_out_cubic(t: f64) -> f64 {
    let t1 = t - 1.0;
    1.0 + t1 * t1 * t1
}

/// 四次缓出：比立方更激进的减速，用于强调终止。
pub fn ease_out_quart(t: f64) -> f64 {
    let t1 = t - 1.0;
    1.0 - t1 * t1 * t1 * t1
}

/// 立方缓入：慢进快出，仅用于淡出。
pub fn ease_in_cubic(t: f64) -> f64 {
    t * t * t
}

/// 立方缓入缓出：用于循环动画（加载指示器等），日常交互不推荐。
pub fn ease_in_out_cubic(t: f64) -> f64 {
    if t < 0.5 {
        4.0 * t * t * t
    } else {
        let u = -2.0 * t + 2.0;
        1.0 - u * u * u / 2.0
    }
}

/// cubic-bezier 缓动求值：给定归一化时间 `x ∈ [0,1]`，返回缓动后进度 `y`。
///
/// 控制点 `(x1,y1,x2,y2)`，端点固定 `P0=(0,0)`、`P3=(1,1)`（同 CSS `cubic-bezier`）。
/// 牛顿迭代反解参数 `u` 使 `X(u)=x`，再求 `Y(u)`；牛顿不收敛时二分兜底。
/// 端点精确：`x=0` 返回 `0.0`、`x=1` 返回 `1.0`（保证满程过渡终值恰等于目标）。
/// 出处：UWP CommandBarFlyout 模板开 / 关曲线（SDK 26100 `generic.xaml`）；
/// 自 Kanesumi `kanesumi-anim/src/easings.rs` 原样回流（BACKFLOW §Ⅰ：曲线不得私藏于调用方）。
pub fn bezier_y(curve: [f32; 4], x: f64) -> f64 {
    let x = x.clamp(0.0, 1.0);
    let [x1, y1, x2, y2] = curve.map(f64::from);
    let bx = |u: f64| {
        let m = 1.0 - u;
        3.0 * m * m * u * x1 + 3.0 * m * u * u * x2 + u * u * u
    };
    let by = |u: f64| {
        let m = 1.0 - u;
        3.0 * m * m * u * y1 + 3.0 * m * u * u * y2 + u * u * u
    };
    let dbx = |u: f64| {
        let m = 1.0 - u;
        3.0 * m * m * x1 + 6.0 * m * u * (x2 - x1) + 3.0 * u * u * (1.0 - x2)
    };
    // 恒等曲线（控制点都在对角线）直接返回。
    if (x1 - y1).abs() < 1e-9 && (x2 - y2).abs() < 1e-9 {
        return x;
    }
    let mut u = x;
    for _ in 0..8 {
        let err = bx(u) - x;
        if err.abs() < 1e-7 {
            break;
        }
        let d = dbx(u);
        if d.abs() < 1e-9 {
            break;
        }
        u = (u - err / d).clamp(0.0, 1.0);
    }
    if (bx(u) - x).abs() > 1e-4 {
        // 牛顿失败：单调区间二分。
        let (mut lo, mut hi) = (0.0, 1.0);
        for _ in 0..40 {
            u = (lo + hi) / 2.0;
            if bx(u) < x {
                lo = u;
            } else {
                hi = u;
            }
        }
    }
    by(u).clamp(0.0, 1.0)
}

#[cfg(test)]
mod tests {
    use super::*;

    /// 正典开曲线（SDK 26100 CommandBarFlyout OpeningStoryboard，0.1,0.9 0.2,1）。
    const OPEN: [f32; 4] = [0.1, 0.9, 0.2, 1.0];
    /// 正典关曲线（同模板 ClosingStoryboard，0.7,0 1,0.5）。
    const CLOSE: [f32; 4] = [0.7, 0.0, 1.0, 0.5];

    #[test]
    fn bezier_endpoints_exact() {
        assert_eq!(bezier_y(OPEN, 0.0), 0.0);
        assert_eq!(bezier_y(OPEN, 1.0), 1.0);
        assert_eq!(bezier_y(CLOSE, 0.0), 0.0);
        assert_eq!(bezier_y(CLOSE, 1.0), 1.0);
    }

    #[test]
    fn bezier_identity_curve() {
        for i in 0..=10 {
            let t = i as f64 / 10.0;
            assert_eq!(bezier_y([0.0, 0.0, 1.0, 1.0], t), t);
        }
    }

    #[test]
    fn bezier_monotonic_and_shape() {
        let mut prev = 0.0;
        for i in 0..=100 {
            let t = i as f64 / 100.0;
            let v = bezier_y(OPEN, t);
            assert!(v >= prev, "t={t} 不增");
            prev = v;
        }
        // 开曲线前段快：t=0.5 时已过半；关曲线前段慢：t=0.5 未过半。
        assert!(bezier_y(OPEN, 0.5) > 0.5);
        assert!(bezier_y(CLOSE, 0.5) < 0.5);
    }

    #[test]
    fn easing_bezier_arm_matches_evaluator() {
        for &t in &[0.25_f64, 0.5, 0.75] {
            assert_eq!(Easing::Bezier(OPEN).apply(t), bezier_y(OPEN, t));
            assert_eq!(Easing::Bezier(CLOSE).apply(t), bezier_y(CLOSE, t));
        }
    }

    #[test]
    fn easing_uwp_arm_matches_apply_uwp() {
        let cases = [
            (UwpEasing::Cubic, EasingMode::EaseOut),
            (UwpEasing::Quadratic, EasingMode::EaseOut),
            (UwpEasing::Quartic, EasingMode::EaseInOut),
        ];
        for (v, m) in cases {
            for t in [0.0_f64, 0.3, 0.5, 1.0] {
                assert_eq!(Easing::Uwp(v, m).apply(t), apply_uwp(t, &v, m));
            }
        }
    }

    #[test]
    fn easing_stays_copy() {
        // Easing 须保持 Copy：Progress 按值持有与返回。
        fn assert_copy<T: Copy>() {}
        assert_copy::<Easing>();
        assert_copy::<UwpEasing>();
    }
}
