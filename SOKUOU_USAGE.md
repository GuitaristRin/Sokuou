# Sokuou Engine 璋冪敤鎵嬪唽

> 鏍稿績鍝插锛?*鍔ㄧ敾涓嶆槸鎾斁锛岃€屾槸绌洪棿鐘舵€佺殑杩炵画瑙ｆ瀽銆?*
> `state 鈫?progress [0.0, 1.0] 鈫?resolved spatial state 鈫?render`
> 鐢ㄦ埛鎰熷彈鍒扮殑涓嶆槸"鍔ㄧ敾鍦ㄦ挱鏀?锛岃€屾槸"绌洪棿鍦ㄨ嚜鐒跺彉鍖?銆?

---

## 1 路 蹇€熷叆闂?

### 瀵煎叆

```rust
use sokuou::{SpringAnim, Progress, Easing, map_range};
```

### 鍦?egui 涓幏鍙?dt

```rust
impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let dt = ctx.input(|i| i.stable_dt) as f64;
        // dt 宸茬粡鏄抚闂存椂闂达紙绉掞級锛岀洿鎺ヤ紶缁?update()
        // 鏃犻渶鎵嬪姩 clamp鈥斺€擲pringAnim 鍐呴儴宸插鐞?dt.min(0.05)

        self.sidebar_anim.update(dt);
        self.page_fade.update(dt);

        // 鏈夊姩鐢昏繍琛屾椂璇锋眰涓嬩竴甯ч噸缁?
        if !self.sidebar_anim.is_steady() || !self.page_fade.is_steady() {
            ctx.request_repaint();
        }
    }
}
```

### 鍏抽敭瑙勫垯

- `update(dt)` 姣忓抚璋冪敤锛岀ǔ鎬佹椂鑷姩璺宠繃锛堟棤寮€閿€锛?
- 鏈夊姩鐢昏繍琛屾椂蹇呴』 `ctx.request_repaint()`锛屽惁鍒?egui 涓嶄細缁х画缁樺埗
- 娓叉煋鍑芥暟鍙?*璇?* `value()`锛屼笉鎿嶄綔鍔ㄧ敾鏈韩

---

## 2 路 閫夋嫨鍝鍘熻

| 鍦烘櫙 | 鍘熻 | 缂撳姩 |
|------|------|------|
| 闈㈡澘婊戝叆/婊戝嚭 | `SpringAnim` | 鍐呯疆鐗╃悊 |
| 绐楀彛/寮圭獥寮瑰叆 | `SpringAnim` | 鍐呯疆鐗╃悊 |
| 椤甸潰鍒囨崲浣嶇Щ | `SpringAnim` | 鍐呯疆鐗╃悊 |
| 閫忔槑搴︽笎鍙?| `Progress` | `EaseOutCubic` |
| 棰滆壊杩囨浮 | `Progress` | `Linear` |
| 鍥炬爣鏃嬭浆锛堥潪寮规€э級 | `Progress` | `EaseOutQuart` |
| 鍔犺浇鎸囩ず鍣ㄥ惊鐜?| `Progress` | `EaseInOutCubic` |

**鍘熷垯**锛氫富杩愬姩锛堜綅缃€佸ぇ灏忥級鐢ㄥ脊绨э紱娆¤灞炴€э紙閫忔槑搴︺€侀鑹诧級鐢?Progress銆?

---

## 3 路 SpringAnim 鍦烘櫙绀轰緥

### 3.1 渚ц竟鏍忓紑鍚?

```rust
// 鍦?MyApp 涓坊鍔犲瓧娈?
pub sidebar_anim: SpringAnim,

// 鍒濆鍖栵紙鍦?MyApp::new()锛?
sidebar_anim: SpringAnim::new(0.5, 0.825, 1.0), // 鍒濆灞曞紑

// 鍒囨崲鏃?
pub fn toggle_sidebar(&mut self) {
    self.sidebar_open = !self.sidebar_open;
    let target = if self.sidebar_open { 1.0 } else { 0.0 };
    self.sidebar_anim.set_target(target);
}

// 娓叉煋鏃讹紙sidebar.rs锛?
fn render(app: &mut MyApp, ctx: &egui::Context) {
    let p = app.sidebar_anim.value();
    let sidebar_width = map_range(p, 0.0, 240.0); // 0px 鈫?240px

    // progress < 0.01 鏃跺彲璺宠繃娓叉煋
    if p < 0.01 { return; }

    egui::SidePanel::left("sidebar")
        .exact_width(sidebar_width as f32)
        .show(ctx, |ui| { /* ... */ });
}
```

### 3.2 椤甸潰鍒囨崲锛堣繘鍦哄姩鐢伙級

```rust
// 鍦?MyApp 涓?
pub page_enter_anim: SpringAnim,

// navigate() 鏃惰Е鍙戣繘鍦?
pub fn navigate(&mut self, page: Page) {
    self.page_history.push(self.current_page.clone());
    self.current_page = page;
    // 浠?0 寮€濮嬶紝寮瑰叆鍒?1
    self.page_enter_anim = SpringAnim::with_target(0.4, 0.8, 0.0, 0.0, 1.0);
}

// 娓叉煋鏃讹紙鍦ㄥ悇椤甸潰 render 鍑芥暟椤堕儴锛?
fn render(app: &mut MyApp, ui: &mut egui::Ui) {
    let p = app.page_enter_anim.value();
    let alpha = ease_out_cubic(p) as f32;
    let offset_y = map_range(p, 20.0, 0.0) as f32; // 鍚戜笂婊戝叆 20px

    ui.add_space(offset_y);
    // 璁剧疆鍏ㄥ眬閫忔槑搴?
    let mut visuals = ui.visuals().clone();
    visuals.override_text_color = Some(visuals.text_color().linear_multiply(alpha));
    ui.set_visuals(visuals);

    // 姝ｅ父娓叉煋椤甸潰鍐呭...
}
```

### 3.3 寮圭獥/瀵硅瘽妗嗗脊鍏?

```rust
// response=0.45, damping=0.7锛氭洿寮癸紝寮鸿皟"寮瑰叆"鎰?
let dialog_anim = SpringAnim::with_target(0.45, 0.7, 0.0, 0.0, 1.0);

// 娓叉煋
let p = dialog_anim.value();
let scale = map_range(p, 0.85, 1.0) as f32; // 85% 鈫?100%
let alpha = ease_out_cubic(p) as f32;

// egui 鐩墠涓嶇洿鎺ユ敮鎸?transform锛屽彲鐢?Rect 鍋忕Щ妯℃嫙锛?
let panel_rect = ui.available_rect_before_wrap();
let center = panel_rect.center();
let scaled_size = panel_rect.size() * scale;
let scaled_rect = egui::Rect::from_center_size(center, scaled_size);
ui.allocate_rect(scaled_rect, egui::Sense::hover());
```

---

## 4 路 Progress 鍦烘櫙绀轰緥

### 4.1 Toast 閫氱煡杩涘満/绂诲満

```rust
// toast.rs 涓殑 Toast 缁撴瀯浣?
pub struct AnimatedToast {
    pub message: String,
    pub level: ToastLevel,
    pub created_at: std::time::Instant,
    pub enter_anim: Progress,  // 杩涘満锛?鈫?锛?
    pub exit_anim: Progress,   // 绂诲満锛?鈫?锛岃Е鍙戝悗寮€濮嬶級
    pub exiting: bool,
}

impl AnimatedToast {
    pub fn new(message: String, level: ToastLevel) -> Self {
        let mut enter_anim = Progress::with_easing(0.25, Easing::EaseOutCubic);
        enter_anim.set_target(1.0); // 绔嬪嵆寮€濮嬭繘鍦?
        Self {
            message, level,
            created_at: std::time::Instant::now(),
            enter_anim,
            exit_anim: Progress::with_easing(0.2, Easing::EaseInCubic),
            exiting: false,
        }
    }

    pub fn dismiss(&mut self) {
        if !self.exiting {
            self.exiting = true;
            self.exit_anim.set_target(1.0);
        }
    }

    pub fn update(&mut self, dt: f64) {
        self.enter_anim.update(dt);
        self.exit_anim.update(dt);
    }

    pub fn is_gone(&self) -> bool {
        self.exiting && self.exit_anim.is_steady()
    }
}

// 娓叉煋鏃?
fn render_toast(toast: &AnimatedToast, ui: &mut egui::Ui) {
    let enter_p = toast.enter_anim.value();
    let exit_p = toast.exit_anim.value();

    // 杩涘満锛氫粠鍙充晶婊戝叆锛涚鍦猴細鍚戜笂婊戝嚭
    let slide_x = map_range(enter_p, 320.0, 0.0);       // 鍙充晶婊戝叆
    let slide_y = map_range(exit_p, 0.0, -60.0);         // 鍚戜笂绂诲満
    let alpha = enter_p * (1.0 - exit_p);                 // 杩涘満浜捣锛岀鍦烘殫涓?

    // 鐢?ui.put() 鎴?painter 鍦ㄧ粷瀵逛綅缃覆鏌?
}
```

### 4.2 鍐呭鍔犺浇鍚庢笎鏄?

```rust
// 鍦?MyApp 鎴栭〉闈㈢姸鎬佷腑
pub content_fade: Progress,

// API 鍝嶅簲鍥炶皟涓?
self.file_list = response.data;
self.content_fade.jump_to(0.0);          // 鍏堥噸缃负涓嶅彲瑙?
self.content_fade.set_target(1.0);       // 鐒跺悗娓愬叆

// 娓叉煋
let alpha = app.content_fade.value() as f32;
ui.set_opacity(alpha); // egui 0.31+ 鏀寔 set_opacity
```

---

## 5 路 map_range 閫熸煡

```rust
use sokuou::map_range;

let p = spring.value(); // 0.0 鈫?1.0

// 閫忔槑搴︼細瀹屽叏涓嶅彲瑙?鈫?瀹屽叏鍙
let alpha = map_range(p, 0.0, 1.0) as f32;

// 鍚戜笂婊戝叆锛堜粠灞忓箷澶?+40px 鍒板師浣嶏級
let offset_y = map_range(p, 40.0, 0.0) as f32;

// 鍚戝乏婊戝叆锛堜晶杈规爮 240px 瀹斤級
let sidebar_w = map_range(p, 0.0, 240.0) as f32;

// 缂╂斁锛?.85 鈫?1.0
let scale = map_range(p, 0.85, 1.0) as f32;

// 棰滆壊鎻掑€硷紙鎵嬪姩瀵规瘡涓€氶亾璋冪敤锛?
let r = map_range(p, from_color.r(), to_color.r());
```

---

## 6 路 涓绘鍔ㄧ敾鍒嗙鍘熷垯

```rust
fn update(&mut self, dt: f64) {
    self.spring_anim.update(dt); // 涓昏繍鍔細寮圭哀鎺у埗浣嶇疆

    // 娆¤灞炴€т粠鍚屼竴涓?progress 娲剧敓锛屼笉鍗曠嫭寤哄姩鐢?
    let p = self.spring_anim.value();
    self.alpha = ease_out_cubic(p);    // 閫忔槑搴﹁窡闅忓脊绨ц繘搴?
    self.scale = map_range(p, 0.9, 1.0); // 缂╂斁璺熼殢寮圭哀杩涘害
}
```

鍚屼竴涓?`progress` 椹卞姩澶氫釜灞炴€э紝淇濊瘉瑙嗚涓€鑷存€р€斺€斾笉瑕佷负閫忔槑搴﹀拰浣嶇疆鍚勫缓涓€涓嫭绔嬪姩鐢汇€?

---

## 7 路 鎺ㄨ崘鍙傛暟閫熸煡

```rust
// 閫氱敤浜や簰锛堢偣鍑汇€侀潰鏉垮垏鎹級
SpringAnim::new(0.5, 0.825, initial)

// 蹇€熶氦浜掞紙鎸夐挳鎸変笅銆佸浘鏍囧弽棣堬級
SpringAnim::new(0.3, 0.6, initial)

// 鎱㈤€熷睍绀猴紙閫氱煡妯箙锛?
SpringAnim::new(0.65, 0.85, initial)

// 寮圭獥寮鸿皟寮瑰叆
SpringAnim::with_target(0.45, 0.7, 0.0, 0.0, 1.0)

// 闈炰氦浜掓贰鍏ワ紙200ms锛?
Progress::with_easing(0.2, Easing::EaseOutCubic)

// 娣″嚭锛堢◢鎱級
Progress::with_easing(0.15, Easing::EaseInCubic)
```

---

## 8 路 鎬ц兘瀹堝垯

```rust
// 1. 绋虫€佷笉閲嶇粯
if !self.anim.is_steady() {
    ctx.request_repaint();
}

// 2. progress 鏋佷綆鏃惰烦杩囨覆鏌?
if app.page_enter_anim.value() < 0.01 { return; }

// 3. 绋虫€佹椂鐢ㄧ簿纭€?
if self.spring.is_steady() {
    draw_at_exact_target();
}

// 4. SpringAnim 鍐呴儴宸?clamp dt.min(0.05)锛屾棤闇€澶栭儴淇濇姢
```

---

## 9 路 棰勭暀 API 璇存槑

`animator.rs` 涓殑 `Animation` trait 鍜?`Animator` 缂栨帓鍣ㄧ洰鍓?*浠呬负棰勭暀瀛樻牴**锛?
涓嶅彲鐢ㄤ簬鐢熶骇浠ｇ爜銆傚綋鍑虹幇浠ヤ笅鐪熷疄闇€姹傛椂鍐嶅疄鐜帮細

- 闇€瑕?闈㈡澘鍗囪捣鍚庡唴瀹规墠寮€濮嬫笎鏄?鐨勪覆琛屼緷璧?
- 闇€瑕佸涓崱鐗囬敊浣嶅欢杩熷叆鍦猴紙stagger锛?

鍙戠幇闇€姹傛椂锛屽厛鍦?`src/sokuou/NOTE.md` 璁板綍鍏蜂綋鍦烘櫙锛屽啀瀹炵幇瀵瑰簲鍘熻銆?

---

## 10 路 璁捐涓嶅彉閲忥紙绂佹浜嬮」锛?

| 绂佹 | 鍘熷洜 |
|------|------|
| `play_animation()` 椋庢牸 API | 杩濆弽鐘舵€侀┍鍔ㄥ師鍒?|
| 鍦ㄦ覆鏌撳嚱鏁颁腑淇敼鍔ㄧ敾鐘舵€?| 娓叉煋鍑芥暟搴斿彧璇?`value()` |
| pop-in锛堢獊鐒跺嚭鐜帮級 | 杩濆弽绌洪棿杩炵画鎬р€斺€旂敤 `Progress::jump_to(0.0)` 鍒濆鍖?|
| 澶氫釜鐙珛鍔ㄧ敾椹卞姩鍚屼竴鍏冪礌鐨勪笉鍚屽睘鎬?| 鏀逛负浠庡崟涓€ `progress` 娲剧敓鎵€鏈夊睘鎬?|
| 蹇樿 `ctx.request_repaint()` | 鍔ㄧ敾浼氬崱鍦ㄧ涓€甯?|
