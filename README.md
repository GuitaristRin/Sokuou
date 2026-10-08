# Sokuou Engine - 即応エンジン

> 进度驱动的空间状态解析器，面向 Rust 应用开发者的动画引擎。
>
> 追求 Apple 级别的流畅与优雅，跨 Linux/Windows/macOS 三平台。

零外部依赖，纯 Rust 数学库。提供三个动画原语：

| 原语 | 用途 | 特性 |
|------|------|------|
| `SpringAnim` | 位移、缩放、面板滑动、页面切换 | 阻尼振荡器解析解，帧率无关，可中断 |
| `Progress` | 透明度、颜色渐变、定时序列 | 时长驱动线性插值，可中断 |
| `MetroAnim` | UWP 缓动短时过渡（Metro 时代） | 11 种缓动变体 × 3 种方向 |

## 快速入门

```rust
use sokuou::{SpringAnim, map_range_clamped};

// 1. 在应用状态中持有动画实例
let mut sidebar_anim = SpringAnim::new(0.4, 0.825, 0.0); // response, damping, initial

// 2. 在事件处理器中设定目标
sidebar_anim.set_target(1.0); // 0.0=收起, 1.0=展开

// 3. 每帧更新（dt 来自渲染循环）
sidebar_anim.update(dt);
if !sidebar_anim.is_steady() { ctx.request_repaint(); }

// 4. 渲染时读取 value()，绝不修改动画状态
let width = map_range_clamped(sidebar_anim.value(), 48.0, 200.0) as f32;
```

## 设计文档

- [SOKUOU_ENGINE.md](SOKUOU_ENGINE.md) - 完整设计书（哲学、数学推导、API 参考）
- [SOKUOU_USAGE.md](SOKUOU_USAGE.md) - 调用手册（快速入门、模式、最佳实践）

## License

MIT
