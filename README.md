# Sokuou（即応エンジン）

> 动画不是播放，而是空间状态的连续解析。
> `state → progress → resolved spatial state → render`

Sokuou 是一个**进度驱动的空间状态解析器**：零依赖、纯 Rust。
其输出为 `f64`，由调用者解为位置、透明度、缩放等视觉属性；
帧循环、渲染管线、布局与输入，皆由调用者自备。

本仓由两部分构成：**动画原语**（根目录 `*.rs`，唯一真源）与**正典**（`docs/`，小文档库）。

## 原语

| 原语 | 用途 | 特性 |
|------|------|------|
| `SpringAnim` | 位移、缩放、面板滑入、页面切换 | 阻尼振荡器解析解；帧率无关、可中断、速度可继承 |
| `Progress` | 透明度、颜色渐变、定时序列 | 时长驱动 `[0,1]`，默认 `EaseOutCubic`，可中断 |
| `MetroAnim` | UWP 规格已定之短时过渡 | `UwpEasing` 全族 × `EasingMode` |
| `Easing` / `UwpEasing` | 缓动曲线族 | 按需添加，经使用验证者入典 |
| `map_range` / `map_range_clamped` | progress 映射到任意值域 | 主次属性同源派生之器 |

## 示例

```rust
use sokuou::{SpringAnim, map_range_clamped};

// 在状态结构中持有动画实例
let mut sidebar = SpringAnim::new(0.4, 0.825, 0.0); // response, damping_ratio, initial

// 事件中设定目标（任意时刻可中断）
sidebar.set_target(1.0); // 0.0 = 收起，1.0 = 展开

// 每帧推进；未稳时请求重绘
sidebar.update(dt);
if !sidebar.is_steady() { request_repaint(); }

// 渲染期只读 value()
let width = map_range_clamped(sidebar.value(), 48.0, 200.0);
```

## 文档

- **正典** [docs/SOKUOU.md](docs/SOKUOU.md) —— 何为 Sokuou 动画、progress 之概念、调用之法、设计不变量。
- **回流** [docs/BACKFLOW.md](docs/BACKFLOW.md) —— 自我迭代之制：发现缺陷、加入新曲线或新原语时，如何改进并回流本仓。
- **历史** [docs/archive/](docs/archive/) —— 旧设计书与调用手册，仅供追溯，不作现行依据。

## License

MIT
