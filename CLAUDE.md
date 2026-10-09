# CLAUDE.md —— Sokuou（即応エンジン）

进度驱动的空间状态解析器：接受状态变化意图与 `dt`，输出 `[0,1]` 的 progress 及解析后的 `f64` 空间状态。
不是渲染器，不是时间线引擎。正典 `docs/SOKUOU.md`；回流之制 `docs/BACKFLOW.md`；硬性规则 `AGENTS.md`。

## 构建与测试

独立仓（本目录）：

```bash
cargo test      # 单测 + doctest
cargo clippy
cargo fmt
```

Ether monorepo 内（submodule）：

```bash
cargo test --workspace -p sokuou
cargo check -p sokuou
```

## 结构

```
mod.rs        公共 API 重导出；map_range / map_range_clamped
progress.rs   时长驱动 [0,1]，默认 EaseOutCubic，可中断
spring.rs     阻尼振荡器解析解（response / damping_ratio / 初速继承）
easing.rs     Easing 枚举 + 缓动自由函数
uwp.rs        UwpEasing 全族 + EasingMode + MetroAnim（UWP 规格过渡）
animator.rs   编排层预留——未经验证，不用于生产
docs/SOKUOU.md    正典
docs/BACKFLOW.md  回流之制
docs/archive/     历史设计书与手册（不作现行依据）
```

## 设计要点

- **Progress**：字段 `from / target / elapsed / duration / easing`；`set_target` 从当前值中断续行；
  `jump_to` 仅用于初始化 / 重置。时长单位秒。
- **SpringAnim**：解析解三支（欠 / 临界 / 过阻尼），帧率无关、无条件稳定；
  `update` 内部 `dt.min(0.05)`，长停顿（> 0.25s）直接吸附；稳态阈值 0.001。
  `set_target_with_velocity` 用于手势速度继承。
- **UwpEasing**：原始函数为 EaseIn 方向，`apply_uwp` 经 `EasingMode` 变换；
  `MetroAnim` 默认 0.25s Quadratic / EaseOut。
- **map_range**：progress 到任意值域之映射；主运动与次要属性同源派生之器。

## 消费方

- Ether monorepo：`shared/sokuou` submodule；workspace 以
  `[patch."https://github.com/GuitaristRin/Sokuou"]` 收敛。
- Kanesumi：`kanesumi-anim` 以 git 依赖消费（权威预设表 `kanesumi-anim/src/presets.rs`）。
- 破坏性变更须同步上述消费方（Ether 侧指针由主仓调度者更新）。

## 约定

- 动代码前先读 `AGENTS.md`（硬性规则）与 `docs/SOKUOU.md`（正典）。
- 注释中文；零外部依赖、纯 Rust、无 `unsafe`、无平台 `cfg`；源码平铺仓根，勿搬入 `src/`。
