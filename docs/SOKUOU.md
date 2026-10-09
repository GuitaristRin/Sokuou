# Sokuou 即応エンジン 正典

> 动画不是播放，而是空间状态的连续解析。
> `state → progress → resolved spatial state → render`

---

## Ⅰ · 定位

### 总述

Sokuou 由二物构成：

- **原语** —— 本仓根目录之 `*.rs`。动画词汇之唯一真源。
- **正典** —— `docs/` 之小文档库。定其法、其律、其回流之制。

Sokuou 不是渲染器。

其不绘制像素，不持窗口，不处理输入，不决定布局，亦不管理应用状态。

Sokuou 之输出，唯一 `f64`。
调用者将此值解为位置、透明度、缩放、旋转——或任何可由进度解析之量。

故：

凡 Ether 扇区内之动画，皆出于 Sokuou。
凡绕过 Sokuou 而自造之通用动画原语与曲线，不得长存——通用者须回流（参 `BACKFLOW.md`）。

### 存在方式

Sokuou 以库嵌入：无守护进程，无全局单例，无注册中心。

```rust
struct DockPanel {
    slide: SpringAnim, // 本面板之滑入
    fade: Progress,    // 本面板之淡入
}
```

每个组件自持其动画实例，构造时即定初始条件，析构即释放，无善后之务。

---

## Ⅱ · 何为 Sokuou 动画

### 非播放，乃解析

Sokuou 无时间线，无关键帧，无 clip，无 `play_animation()`。

```
state → progress → resolved spatial state → render
```

而非：

```
play_animation() → timeline → render
```

动画非「被播放」，乃系统状态被连续计算后之结果。
用户不应觉得「动画正在播放」，而应觉得「空间正在变化」。

### 连续

任意 progress 值，皆对应唯一确定之空间状态。

系统不得有 pop-in、骤生、骤灭、断点。
元素不得凭空出现，亦不得骤然消失；其进退，皆以 progress 由 0 至 1、由 1 至 0 解析之。

### 可中断

任意时刻可改其目标。
中断时不得归零重演，而当承当前之位置与速度，续行其路。

---

## Ⅲ · Progress

### 本质

```
progress ∈ [0.0, 1.0]
```

progress 不是时间，不是帧数，不是「播放到百分之几」。

progress 是**当前空间状态值**。
`0.25` 不谓「已播四分之一」，而谓「系统此刻真实处于四分之一之空间状态」。

### 时间之职

```
time     → 决定 progress 之变化速度
progress → 决定空间状态
```

时间可以影响速度、阻尼、惯性；
空间状态本身，永远只由 progress 决定。

故任意时刻之渲染结果可写作 `render_state = resolve(progress)`。
`resolve` 须确定、连续、可停留于任意值。

### 一 progress，多属性

同一元素之诸视觉属性，当由同一 progress 派生。

主运动（位移、缩放）以 `SpringAnim` 或 `Progress` 生成 progress；
次要属性（透明度、颜色）自同一 progress 以 `map_range` 或缓动函数派生。

不得为同一元素之不同属性各立动画——那会使空间失去一致。

### 主次之别

| 属性 | 原语 | 缓动 |
|---|---|---|
| 位移 / 缩放 / 面板滑入 / 页面切换 | `SpringAnim` | 内置物理 |
| 透明度 / 颜色 / 定时序列 | `Progress` | 默认 `EaseOutCubic`；颜色用 `Linear` |
| UWP 规格已定之短时过渡 | `MetroAnim` | `UwpEasing` × `EasingMode` |

原则：主运动用弹簧；次要属性用时长驱动；凡 UWP 规格已定之过渡，从其规格。
非交互淡入可不用弹簧（`Progress` + `EaseOutCubic`）；循环者用 `EaseInOutCubic`，日常交互不推荐。

---

## Ⅳ · 调用之法

### 循环契约

1. 于状态结构中持有动画实例——字段，非全局。
2. 事件处理器中 `set_target()`；初始化或重置用 `jump_to()`。
3. 每帧 `update(dt)`，`dt` 为秒。
4. 有动画未稳（`!is_steady()`）时，向宿主请求重绘；稳态不重绘。
5. 渲染期只读 `value()`，绝不修改动画状态。

```rust
use sokuou::{SpringAnim, map_range_clamped};

let mut slide = SpringAnim::new(0.5, 0.825, 0.0); // response, damping_ratio, initial
slide.set_target(1.0);                             // 任意时刻可中断
slide.update(dt);                                  // 每帧；内部已保护 dt
if !slide.is_steady() { request_repaint(); }       // 稳态不重绘
let x = map_range_clamped(slide.value(), 0.0, 240.0); // 渲染期只读
```

签名之权威在源码 rustdoc（`cargo doc --open`）；正典定法与律，不逐条罗列签名。
`SpringAnim` 内部已 clamp dt，并对长停顿直接吸附；调用者无需重复保护。

### 手势

手势结束，以手势速度为弹簧初速：`set_target_with_velocity(target, velocity)`。
物自有其动量，不因指离而骤停。

### 参数族

弹簧（`SpringAnim::new(response, damping_ratio, initial)`）：

| 场景 | response | damping_ratio |
|---|---|---|
| 页面转场 | 0.40 | 0.80 |
| 通用交互 / 面板滑入 | 0.50 | 0.825 |
| 弹窗弹入 | 0.45 | 0.70 |
| 快速交互 / 图标反馈 | 0.30 | 0.60 |
| 慢速展示 / 通知 | 0.65 | 0.85 |

时长驱动（`Progress`）：非交互淡入 0.2s `EaseOutCubic`；淡出 0.15s `EaseInCubic`；
UWP 规格过渡默认 0.25s `Quadratic` / `EaseOut`。

Ether / Kanesumi 扇区另有权威预设表（`kanesumi-anim/src/presets.rs`）；调用方已有预设者，从其预设。

### 宿主之职

Sokuou 不提供帧循环、渲染管线、布局与输入。

凡构建动画系统者——人或 agent——须自备宿主：取 dt，驱动 `update`，读 progress，映射为视觉属性。
以本正典为法，按所求之方向构筑。

方向不明时，当向要求者问明。
不得因「无现成求解器与渲染管线」而弃用 Sokuou，亦不得以临时私有实现替代之。
凡此过程中所得之通用改进，须回流 Sokuou（参 `BACKFLOW.md`）。

### 禁止

| 禁止 | 缘由 |
|---|---|
| `play_animation()` 风格之 API | 违状态驱动 |
| 渲染函数中修改动画状态 | 渲染只读 |
| pop-in（骤现） | 违空间连续；以 `jump_to` 初始化 |
| 一元素多动画各驱其不同属性 | 违一致性 |
| 绕过 Sokuou 自造通用曲线 / 原语 | 违单一真源 |
| 将 Sokuou 当作渲染器或时间线引擎 | 违定位 |

---

## Ⅴ · 设计不变量

| 不变量 | 含义 |
|---|---|
| 可中断 | 任意时刻可改目标、反向、接管、回弹，不锁定 |
| 连续 | 无 pop-in、无骤灭；一切经 progress 过渡 |
| 渐隐即转移 | 淡出者必伴随另一元素之淡入，空间存在感守恒 |
| 进度不伪 | progress 须来自真实时间与真实物理，不得缩放伪造 |
| 状态驱动 | 渲染只读 progress，不是 `play_animation()` |
| 确定 | 任意 progress 值有唯一解析结果 |
| 帧率无关 | 弹簧用解析解；轨迹只由物理参数决定，与帧率无关 |
| 空间真实 | 用户不应觉得「动画在播放」，而应觉得「空间在变化」 |
