# CLAUDE.md —— Sokuou（即応エンジン）

<!-- BEGIN workbench-pointer · v1 · 勿手改本块（由 GuitaristRin/Projects 统一维护；改模板请改 templates/repo-AGENTS-block.md） -->
> **本文件只写本仓特有的规则。跨仓通用约定不在本仓重复**，唯一真源是工作台文档库：
> - Windows：`%USERPROFILE%\Documents\Projects\AGENTS.md`
> - Linux：`~/projects/AGENTS.md` → `~/projects/Arcturius/AGENTS.md`（按序取第一个存在的）
> - 远端：<https://github.com/GuitaristRin/Projects/blob/main/AGENTS.md>
>
> 动手之前先读它。**读不到就在报告里写明「工作台未读到」**，然后按本文件执行，不要猜。
> 新发现的通用规则先回流工作台（`docs/09-回流与同步.md`），本文件只留引用，不抄正文。
> 冲突优先级：用户当前明确指令 > 本仓特有规则 > 工作台通用规则。
<!-- END workbench-pointer -->

进度驱动的空间状态解析器：接受目标变化与 `dt`，输出可由宿主映射为视觉属性的 `f64`。
`[0,1]` 为常用端点约定，弹簧等可过冲。不是渲染器，不是时间线引擎。
文档入口 `docs/README.md`；修改本仓遵循 `AGENTS.md`；在消费方使用时遵循调用指南与消费方工作流。

## 构建与测试

独立仓（本目录）：

```bash
cargo test      # 单测 + doctest
cargo clippy
cargo fmt
```

由宿主 Cargo workspace 管理时，在宿主仓根按其成员配置执行，例如：

```bash
cargo test -p sokuou
cargo check -p sokuou
```

## 结构

```
mod.rs        公共 API 重导出；map_range / map_range_clamped
progress.rs   时长驱动标量过渡，常用端点 [0,1]，默认 EaseOutCubic，可中断
spring.rs     阻尼振荡器解析解（response / damping_ratio / 初速继承）
easing.rs     Easing 枚举 + 缓动自由函数
uwp.rs        UwpEasing 全族 + EasingMode + MetroAnim（UWP 规格过渡）
animator.rs   编排层预留——未经验证，不用于生产
docs/README.md       文档库入口与 Agent 接入方式
docs/SOKUOU.md       正典：概念、契约与边界
docs/INTEGRATION.md  调用、宿主适配、验证与经验总结
docs/BACKFLOW.md     经验反馈、问题报告与贡献路径
docs/archive/        历史设计书与手册（不作现行依据）
```

## 设计要点

- **Progress**：字段 `from / target / elapsed / duration / easing`；`set_target` 从当前值中断续行；
  `jump_to` 用于初始化 / 重置或有意即时到位。时长单位秒，不保证打断时速度连续。
- **SpringAnim**：解析解三支（欠 / 临界 / 过阻尼），从当前值与速度中断续行；
  `update` 内部 `dt.min(0.05)`，长停顿（> 0.25s）直接吸附；稳态阈值 0.001。
  `set_target_with_velocity` 用于手势速度继承，速度单位须为动画值 / 秒。无 `jump_to`，即时到目标用 `snap`。
- **UwpEasing**：原始函数为 EaseIn 方向，`apply_uwp` 经 `EasingMode` 变换；
  `MetroAnim` 默认 0.25s Quadratic / EaseOut。
- **map_range**：progress 到任意值域之映射；主运动与次要属性同源派生之器。

## 宿主接入与经验反馈

- 每个组件或状态持有者保存实例；事件改目标并唤起首帧，更新阶段推进，渲染只读。
- 同一语义过渡的属性同源派生，受限属性在映射时钳制；独立交互状态可以独立驱动。
- 动画刚到稳态仍须绘制最终值；退出完成后再卸载；稳态不因该动画继续请求帧。
- `Progress` / `MetroAnim` 不共享弹簧的 dt 钳制与长停顿吸附策略。
- 调用所得在任务总结中简述；可复用经验、缺陷与候选能力按 `docs/BACKFLOW.md` 整理为报告或改进。
- 破坏性变更写明迁移办法，由各消费方维护者安排依赖升级。

## 约定

- 动代码前先读 `AGENTS.md`（硬性规则）与 `docs/SOKUOU.md`（正典）。
- 注释中文；零外部依赖、纯 Rust、无 `unsafe`、无平台 `cfg`；源码平铺仓根，勿搬入 `src/`。
