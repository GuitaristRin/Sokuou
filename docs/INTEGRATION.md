# Sokuou 调用指南

> 先定状态，次定其解，再择驱动。动画随交互而生。

本文面向在任意宿主中使用 Sokuou 的开发者与 Agent。
概念与边界见 [SOKUOU.md](SOKUOU.md)，反馈与贡献见 [BACKFLOW.md](BACKFLOW.md)。

## Ⅰ · 从交互意图到原语

构建动画时，依次回答：

1. **状态为何？** 如收起 / 展开、未选 / 已选、拖拽 / 释放。先明确目标与初始状态。
2. **如何解析？** 写出 `resolve(progress)`，确定位置、透明度、缩放等属性的端点与约束。
3. **如何变化？** 主运动优先 `SpringAnim`；固定时长过渡用 `Progress`；明确遵循 UWP 规格者用 `MetroAnim`。
4. **谁来驱动？** 由组件或其状态持有者保存动画；宿主负责时间、事件、重绘与生命周期。
5. **如何收尾？** 明确反向、快速重复输入、退出卸载、后台恢复及减少动态效果时的行为。

已有交互规格与项目预设时，从其约定；缺少信息但不影响结构时，采用正典的默认建议并说明假设。
只有会改变交互语义的歧义才需要先问明，不应为每个参数停下来询问。
宿主尚未提供帧循环或布局适配时，在宿主补齐相应机制。

同一语义过渡的属性共享 progress。例如面板滑入时，位置、缩放、透明度均由展开进度派生。
独立状态可以独立驱动，例如同一按钮的 hover 与按压反馈；不要仅因它们属于同一元素便强行合并。

## Ⅱ · 宿主循环与生命周期

- **初始化**：从真实初始状态构造。`SpringAnim::new(..., initial)`；`Progress` / `MetroAnim` 可用 `jump_to(initial)`。
- **事件**：目标确实变化时调用 `set_target()`，并唤起首帧重绘。不要在每次渲染时重设同一目标，重设可能重启过渡。
- **更新**：以宿主单调时钟得到的有限、非负秒数调用 `update(dt)`，每实例每更新步推进一次。
- **渲染**：只读 `value()`，经确定的映射得到视觉属性，不在渲染期间推进或重设动画。
- **续帧**：尚有 `!is_steady()` 的动画时请求下一帧。即使本步刚到稳态，也须渲染最终值；稳态仅表示动画无需再续帧。
- **退出**：先把目标设为退出态，保留实例和待绘制内容，稳态后再卸载。不可先移除再期待淡出。

`SpringAnim` 会将单步时间限为 0.05s，超过 0.25s 的停顿直接吸附到目标。
`Progress` / `MetroAnim` 按传入时间累计直至时长终点；它们不共享弹簧的停顿策略。
宿主应明确后台恢复行为，不要声称三者拥有相同的时间策略。详见正典 §Ⅳ。

### 一例：面板的宿主适配

以下为不依赖 UI 框架的完整 Rust 示例。参数采用正典 §Ⅳ 的通用交互建议，端点仅作演示。
宿主在事件中调用 `set_open`，在更新阶段调用 `tick`，在渲染阶段调用 `resolve`。
`set_open` 返回是否需要唤起首帧，`tick` 返回是否需要继续请求下一帧；当前帧始终绘制更新后的状态。

```rust
use sokuou::{map_range, map_range_clamped, SpringAnim};

struct Panel {
    open: bool,
    motion: SpringAnim,
}

impl Panel {
    fn new(open: bool) -> Self {
        Self {
            open,
            motion: SpringAnim::new(0.5, 0.825, if open { 1.0 } else { 0.0 }),
        }
    }

    fn set_open(&mut self, open: bool) -> bool {
        if self.open == open {
            return false;
        }
        self.open = open;
        self.motion.set_target(if open { 1.0 } else { 0.0 });
        true
    }

    fn tick(&mut self, dt: f64) -> bool {
        self.motion.update(dt);
        !self.motion.is_steady()
    }

    fn resolve(&self) -> (f64, f64) {
        let p = self.motion.value();
        let x = map_range(p, -240.0, 0.0); // 位移保留弹簧过冲。
        let alpha = map_range_clamped(p, 0.0, 1.0); // 透明度限制合法值域。
        (x, alpha)
    }

    fn can_unmount(&self) -> bool {
        !self.open && self.motion.is_steady()
    }
}

fn main() {
    let mut panel = Panel::new(false);
    assert!(panel.set_open(true)); // 宿主据此请求首帧。
    let _needs_next_frame = panel.tick(1.0 / 60.0); // 仅演示一次更新；实际 dt 来自宿主时钟。
    let (_x, _alpha) = panel.resolve(); // 渲染只读。
    assert!(!panel.can_unmount());
}
```

### 打断与手势

`SpringAnim::set_target()` 从当前值与速度续行；`Progress` / `MetroAnim` 从当前值重新计时，不保证速度连续。
手势释放可用 `set_target_with_velocity()`，速度单位须与动画值一致：若动画为归一化展开进度，则像素 / 秒须先换算为 progress / 秒。
直接跟随拖拽期间的位置与输入仍归宿主管理，释放后的回弹可交由 Sokuou；不要调用不存在的弹簧 `jump_to()`。

### 可访问性与即时切换

减少动态效果、首次布局、恢复状态等场景可由宿主选择即时到位或更温和的过渡。
`SpringAnim` 可在设定目标后 `snap()`；`Progress` / `MetroAnim` 可 `jump_to(target)`。
这些是有意的交互策略，应与偶然的 pop-in 区分；同时保持最终状态与卸载逻辑正确。

## Ⅲ · 如何验证调用

按实际交互检查，不必为每个简单动画增设一套测试框架：

- 目标变化会唤起首帧；稳态停止续帧，最终帧仍绘制。
- 半途反向与快速重复输入，从当前状态续行，无归零重演。
- 同一语义过渡的属性同源；透明度等受限值不因弹簧过冲越界。
- 退出内容在过渡完成前仍可渲染；初始状态与真实业务状态相符。
- 在宿主支持的帧率与后台恢复场景中，行为符合所选原语的时间策略。

涉及库本身的行为或数值修改，必须按本仓守则补回归测试。
报告应区分编译 / 自动测试、实际运行观察与尚未验证的判断；编译成功不能代替视觉检查。

## Ⅳ · 把调用所得留下来

动画任务收尾时，在调用方现有任务总结、PR 或项目笔记中，用几行记录真实所得即可。
已有同类记录便更新，不必为没有新发现的任务创建重复档案。

```text
场景与版本：交互目标；Sokuou 版本 / 提交；宿主或框架。
实现选择：原语、参数与依据；progress 如何派生属性；必要的宿主策略。
验证结果：实际检查了哪些打断、退出、帧率或停顿场景；还有哪些未验证。
调用所得：有效做法、遇到的摩擦或限制、最小复现 / 证据（如有）。
回流判断：留在项目的专用经验 / 值得共享的用法 / 文档问题 / 库缺陷 / 候选能力。
后续状态：无需跟进，或关联的本地记录、上游 Issue / PR；尚未提交时如实注明。
```

经验不只包括缺陷：一次有效的参数选择、对宿主边界的澄清、避免重复驱动的做法，都可能帮助下一位使用者。
先判断可复用性，再按 [BACKFLOW.md](BACKFLOW.md) 选择反馈方式。
Agent 应主动识别并整理这些候选，不把“发现候选”写成“已回流”，也不因没有上游写权限而丢失经验。
