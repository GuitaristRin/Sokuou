# Sokuou（即応エンジン）

> 动画不是播放，而是空间状态的连续解析。
> `state → progress → resolved spatial state → render`

Sokuou 是一个**进度驱动的空间状态解析器**：零依赖、纯 Rust。
其输出为 `f64`，由调用者解为位置、透明度、缩放等视觉属性；
帧循环、渲染管线、布局与输入，皆由调用者自备。

本仓由两部分构成：**动画原语**（根目录 `*.rs`，公共实现）与**文档库**（`docs/`，动画思路、调用实践与经验回流）。
面向任意 Rust 应用与 UI 宿主，不依赖特定框架或项目。开发者与 Agent 均可从[文档库入口](docs/README.md)开始。

## 接入

在应用的 `Cargo.toml` 中添加 git 依赖：

```toml
[dependencies]
sokuou = { git = "https://github.com/GuitaristRin/Sokuou" }
```

需要固定版本时，以 `rev` 指定已验证的提交，并查阅该提交对应的文档。
也可使用本地 path 依赖或 submodule；宿主负责自己的帧循环与集成方式。

## 原语

| 原语 | 用途 | 特性 |
|------|------|------|
| `SpringAnim` | 位移、缩放、面板滑入、页面切换 | 阻尼振荡器解析解；可中断、速度可继承；时间保护策略见正典 |
| `Progress` | 独立淡入淡出、颜色渐变、固定时长过渡 | 通常约定端点为 `[0,1]`，默认 `EaseOutCubic`，从当前值中断续行 |
| `MetroAnim` | UWP 规格已定之短时过渡 | `UwpEasing` 全族 × `EasingMode` |
| `Easing` / `UwpEasing` | 缓动曲线族 | 按需添加，经使用验证者入典 |
| `map_range` / `map_range_clamped` | progress 映射到任意值域 | 主次属性同源派生之器 |

## 示例

以下为宿主循环中的片段；`dt` 与 `request_repaint()` 由宿主提供。
完整、框架无关的 Rust 示例及退出生命周期见[调用指南](docs/INTEGRATION.md)。

```rust
use sokuou::{SpringAnim, map_range_clamped};

// 在状态结构中持有动画实例
let mut sidebar = SpringAnim::new(0.4, 0.825, 0.0); // response, damping_ratio, initial

// 事件中设定目标（任意时刻可中断）
sidebar.set_target(1.0); // 0.0 = 收起，1.0 = 展开；仅在目标变化时设定
request_repaint();      // 事件唤起首帧

// 更新阶段推进；未稳时请求下一帧，本帧始终绘制更新后的值
sidebar.update(dt);
if !sidebar.is_steady() { request_repaint(); }

// 渲染期只读 value()
let width = map_range_clamped(sidebar.value(), 48.0, 200.0);
```

## 文档与 Agent 入口

- **导航** [docs/README.md](docs/README.md) —— 阅读顺序、文档职责，以及如何在调用方项目指引中接入 Agent。
- **正典** [docs/SOKUOU.md](docs/SOKUOU.md) —— 何为 Sokuou 动画、progress 之概念、调用之法、设计不变量。
- **调用** [docs/INTEGRATION.md](docs/INTEGRATION.md) —— 从交互意图到原语、宿主适配、验证与调用经验总结。
- **回流** [docs/BACKFLOW.md](docs/BACKFLOW.md) —— 分享用法、报告问题、修正文档或贡献原语；无需先完成代码修复才参与。
- **本仓协作** [AGENTS.md](AGENTS.md) · [CLAUDE.md](CLAUDE.md) —— 修改 Sokuou 本仓时的规则与架构导览。
- **历史** [docs/archive/](docs/archive/) —— 旧设计书与调用手册，仅供追溯，不作现行依据。

使用时若有可复用所得，欢迎通过 [Issue](https://github.com/GuitaristRin/Sokuou/issues) 或 PR 带回。
采用 Sokuou 不以贡献代码为前提；一次清楚的使用报告同样有价值。

## License

MIT
