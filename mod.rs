// Sokuou Engine — 即応エンジン
// 面向 Rust 应用开发者的动画引擎，进度驱动的空间状态解析器。
//
// 设计书：SOKUOU_ENGINE.md
// 调用手册：SOKUOU_USAGE.md

pub mod animator;
pub mod easing;
pub mod progress;
pub mod spring;
pub mod uwp;

pub use animator::{Animation, Animator};
pub use easing::{
    ease_in_cubic, ease_in_out_cubic, ease_out_cubic, ease_out_quad, ease_out_quart, Easing,
};
pub use progress::Progress;
pub use spring::SpringAnim;
pub use uwp::{apply_uwp, EasingMode, MetroAnim, UwpEasing};

/// 将 progress [0.0, 1.0] 映射到任意值域 [from, to]。
///
/// ```
/// use sokuou::map_range;
///
/// // 同一进度值（例如弹簧动画的当前值）映射到不同视觉属性。
/// let progress = 0.5_f64;
/// let alpha = map_range(progress, 0.0, 1.0);   // 0.0 → 1.0
/// let slide = map_range(progress, 100.0, 0.0); // 从底部滑入（值域可逆）
/// let scale = map_range(progress, 0.85, 1.0);  // 85% → 100%
/// assert_eq!(alpha, 0.5);
/// assert_eq!(slide, 50.0);
/// assert!((scale - 0.925).abs() < 1e-9);
/// ```
pub fn map_range(progress: f64, from: f64, to: f64) -> f64 {
    from + (to - from) * progress
}

/// 同 map_range，但先将 progress clamp 到 [0.0, 1.0]。
pub fn map_range_clamped(progress: f64, from: f64, to: f64) -> f64 {
    map_range(progress.clamp(0.0, 1.0), from, to)
}
