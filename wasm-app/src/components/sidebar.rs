use leptos::prelude::*;
use leptos::html;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement};
use crate::animation::scramble_text::ScrambleText;

/// Matrix rain canvas animation — ported from the original JS.
/// Runs via requestAnimationFrame in Rust/WASM.
#[component]
pub fn Sidebar() -> impl IntoView {
    // Scramble text state
    let (name_text, set_name_text) = signal("Vidya Bodepudi".to_string());
    let (is_scrambling, set_is_scrambling) = signal(false);

    let on_hover = move |_| {
        if !is_scrambling.get() {
            run_scramble(set_name_text.clone(), set_is_scrambling.clone(), 600.0);
        }
    };
    let (nav_opacity, set_nav_opacity) = signal(0.0_f64);
    let (nav_transform, set_nav_transform) = signal(20.0_f64);
    let (socials_opacity, set_socials_opacity) = signal(0.0_f64);
    let (avatar_scale, set_avatar_scale) = signal(0.0_f64);
    let (avatar_opacity, set_avatar_opacity) = signal(0.0_f64);
    let (role_opacity, set_role_opacity) = signal(0.0_f64);
    let (active_nav, set_active_nav) = signal("home".to_string());

    let canvas_ref = NodeRef::<html::Canvas>::new();

    // Boot sequence timeline — orchestrated entrance
    Effect::new(move |_| {
        let window = web_sys::window().unwrap();
        let performance = window.performance().unwrap();

        // Phase 1: Avatar fade in at t=200ms
        {
            let start = performance.now();
            let cb = Closure::wrap(Box::new(move || {
                let elapsed = web_sys::window().unwrap().performance().unwrap().now() - start;
                let delay = 200.0;
                let duration = 600.0;
                if elapsed > delay {
                    let t = ((elapsed - delay) / duration).min(1.0);
                    let eased = 1.0 - (1.0 - t).powi(3); // ease-out-cubic
                    set_avatar_opacity.set(eased);
                    set_avatar_scale.set(0.5 + 0.5 * eased);
                }
            }) as Box<dyn Fn()>);

            let interval_id = window
                .set_interval_with_callback_and_timeout_and_arguments_0(
                    cb.as_ref().unchecked_ref(),
                    16,
                )
                .unwrap();

            // Stop after animation completes
            let window2 = window.clone();
            let cleanup = Closure::wrap(Box::new(move || {
                window2.clear_interval_with_handle(interval_id);
            }) as Box<dyn Fn()>);
            window
                .set_timeout_with_callback_and_timeout_and_arguments_0(
                    cleanup.as_ref().unchecked_ref(),
                    1000,
                )
                .unwrap();
            cb.forget();
            cleanup.forget();
        }

        // Phase 2: Name scramble at t=500ms
        {
            let set_name = set_name_text.clone();
            let set_scrambling = set_is_scrambling.clone();
            let window_clone = window.clone();
            let boot_scramble = Closure::wrap(Box::new(move || {
                run_scramble(set_name, set_scrambling, 800.0);
            }) as Box<dyn Fn()>);
            
            window_clone
                .set_timeout_with_callback_and_timeout_and_arguments_0(
                    boot_scramble.as_ref().unchecked_ref(),
                    500,
                )
                .unwrap();
            boot_scramble.forget();
        }

        // Phase 3: Role tagline at t=800ms
        {
            let start = performance.now();
            let cb = Closure::wrap(Box::new(move || {
                let elapsed = web_sys::window().unwrap().performance().unwrap().now() - start;
                let delay = 800.0;
                let duration = 400.0;
                if elapsed > delay {
                    let t = ((elapsed - delay) / duration).min(1.0);
                    let eased = 1.0 - (1.0 - t).powi(3);
                    set_role_opacity.set(eased);
                }
            }) as Box<dyn Fn()>);

            let interval_id = window
                .set_interval_with_callback_and_timeout_and_arguments_0(
                    cb.as_ref().unchecked_ref(),
                    16,
                )
                .unwrap();

            let window2 = window.clone();
            let cleanup = Closure::wrap(Box::new(move || {
                window2.clear_interval_with_handle(interval_id);
            }) as Box<dyn Fn()>);
            window
                .set_timeout_with_callback_and_timeout_and_arguments_0(
                    cleanup.as_ref().unchecked_ref(),
                    1400,
                )
                .unwrap();
            cb.forget();
            cleanup.forget();
        }

        // Phase 4: Nav links stagger at t=1200ms (4 links, 80ms apart)
        {
            let start = performance.now();
            let cb = Closure::wrap(Box::new(move || {
                let elapsed = web_sys::window().unwrap().performance().unwrap().now() - start;
                let delay = 1200.0;
                let duration = 400.0;
                if elapsed > delay {
                    let t = ((elapsed - delay) / duration).min(1.0);
                    let eased = 1.0 - (1.0 - t).powi(3);
                    set_nav_opacity.set(eased);
                    set_nav_transform.set(20.0 * (1.0 - eased));
                }
            }) as Box<dyn Fn()>);

            let interval_id = window
                .set_interval_with_callback_and_timeout_and_arguments_0(
                    cb.as_ref().unchecked_ref(),
                    16,
                )
                .unwrap();

            let window2 = window.clone();
            let cleanup = Closure::wrap(Box::new(move || {
                window2.clear_interval_with_handle(interval_id);
            }) as Box<dyn Fn()>);
            window
                .set_timeout_with_callback_and_timeout_and_arguments_0(
                    cleanup.as_ref().unchecked_ref(),
                    2000,
                )
                .unwrap();
            cb.forget();
            cleanup.forget();
        }

        // Phase 5: Socials at t=1600ms
        {
            let start = performance.now();
            let cb = Closure::wrap(Box::new(move || {
                let elapsed = web_sys::window().unwrap().performance().unwrap().now() - start;
                let delay = 1600.0;
                let duration = 400.0;
                if elapsed > delay {
                    let t = ((elapsed - delay) / duration).min(1.0);
                    let eased = 1.0 - (1.0 - t).powi(3);
                    set_socials_opacity.set(eased);
                }
            }) as Box<dyn Fn()>);

            let interval_id = window
                .set_interval_with_callback_and_timeout_and_arguments_0(
                    cb.as_ref().unchecked_ref(),
                    16,
                )
                .unwrap();

            let window2 = window.clone();
            let cleanup = Closure::wrap(Box::new(move || {
                window2.clear_interval_with_handle(interval_id);
            }) as Box<dyn Fn()>);
            window
                .set_timeout_with_callback_and_timeout_and_arguments_0(
                    cleanup.as_ref().unchecked_ref(),
                    2200,
                )
                .unwrap();
            cb.forget();
            cleanup.forget();
        }
    });

    // Matrix rain effect on canvas
    Effect::new(move |_| {
        let Some(canvas) = canvas_ref.get() else { return };
        let canvas_el: &HtmlCanvasElement = canvas.as_ref();

        let parent = canvas_el.parent_element().unwrap();
        canvas_el.set_width(parent.client_width() as u32);
        canvas_el.set_height(parent.client_height() as u32);

        let ctx = canvas_el
            .get_context("2d")
            .unwrap()
            .unwrap()
            .dyn_into::<CanvasRenderingContext2d>()
            .unwrap();

        let katakana = "ｱｲｳｴｵｶｷｸｹｺｻｼｽｾｿﾀﾁﾂﾃﾄﾅﾆﾇﾈﾉﾊﾋﾌﾍﾎﾏﾐﾑﾒﾓﾔﾕﾖﾗﾘﾙﾚﾛﾜﾝ";
        let nums = "0123456789";
        let symbols = ":・.\"=*+-<>¦|";
        let all_chars: String = format!("{}{}{}", katakana, nums, symbols);
        let chars: Vec<char> = all_chars.chars().collect();
        let font_size = 16.0_f64;

        let colors: Vec<&str> = vec![
            "#00ff41", "#00ff41", "#00ff41", "#00ff41", "#00ff41",
            "#00ffff", "#ffff00", "#ff1493", "#ff6600", "#bf00ff", "#ff0044", "#0088ff",
        ];

        let lead_colors: Vec<(&str, &str)> = vec![
            ("#00ff41", "#aaffaa"),
            ("#00ffff", "#ccffff"),
            ("#ffff00", "#ffffaa"),
            ("#ff1493", "#ffaaff"),
            ("#ff6600", "#ffd699"),
            ("#bf00ff", "#ddaaff"),
            ("#ff0044", "#ffaaaa"),
            ("#0088ff", "#aaddff"),
        ];

        let w = canvas_el.width() as f64;
        let h = canvas_el.height() as f64;
        let columns = (w / font_size).floor() as usize;

        // Initialize drops at random positions
        let drops = js_sys::Array::new();
        for _ in 0..columns {
            drops.push(&JsValue::from_f64(
                (js_sys::Math::random() * (h / font_size)).floor(),
            ));
        }

        let window = web_sys::window().unwrap();
        let cb = Closure::wrap(Box::new(move || {
            // Semi-transparent overlay for trail effect
            ctx.set_fill_style_str("rgba(3, 5, 8, 0.04)");
            ctx.fill_rect(0.0, 0.0, w, h);

            ctx.set_font(&format!("{}px monospace", font_size));

            for i in 0..columns {
                let char_idx = (js_sys::Math::random() * chars.len() as f64).floor() as usize;
                let ch = chars[char_idx % chars.len()];
                let x = i as f64 * font_size;
                let drop_val = drops.get(i as u32).as_f64().unwrap_or(0.0);
                let y = drop_val * font_size;

                // Lead character — bright version
                let color_idx = (js_sys::Math::random() * colors.len() as f64).floor() as usize;
                let base_color = colors[color_idx % colors.len()];
                let lead_color = lead_colors
                    .iter()
                    .find(|(c, _)| *c == base_color)
                    .map(|(_, l)| *l)
                    .unwrap_or("#aaffaa");

                ctx.set_fill_style_str(lead_color);
                ctx.fill_text(&ch.to_string(), x, y).ok();

                // Trail character
                if drop_val > 1.0 {
                    let trail_idx = (js_sys::Math::random() * chars.len() as f64).floor() as usize;
                    let trail_ch = chars[trail_idx % chars.len()];
                    let trail_color_idx =
                        (js_sys::Math::random() * colors.len() as f64).floor() as usize;
                    ctx.set_fill_style_str(colors[trail_color_idx % colors.len()]);
                    ctx.fill_text(&trail_ch.to_string(), x, y - font_size).ok();
                }

                // Reset drop
                if y > h && js_sys::Math::random() > 0.95 {
                    drops.set(i as u32, JsValue::from_f64(0.0));
                } else {
                    drops.set(i as u32, JsValue::from_f64(drop_val + 1.0));
                }
            }
        }) as Box<dyn Fn()>);

        window
            .set_interval_with_callback_and_timeout_and_arguments_0(
                cb.as_ref().unchecked_ref(),
                33,
            )
            .unwrap();
        cb.forget();
    });

    view! {
        <aside class="sidebar">
            <canvas node_ref=canvas_ref id="matrix-canvas"></canvas>
            <div class="sidebar-content">
                <div class="profile">
                    <img
                        src="https://avatars.githubusercontent.com/u/9919?s=280&v=4"
                        alt="Profile Picture"
                        class="avatar"
                        style=move || format!(
                            "opacity: {}; transform: scale({})",
                            avatar_opacity.get(),
                            avatar_scale.get()
                        )
                    />
                    <h1 
                        class="glitch-text" 
                        data-text=move || name_text.get()
                        on:mouseenter=on_hover
                    >
                        {move || name_text.get()}
                    </h1>
                    <p class="role" style=move || format!("opacity: {}", role_opacity.get())>
                        "> Security Researcher | AI Enthusiast | Open Source Advocate"
                    </p>
                </div>
                <nav
                    class="nav-links"
                    style=move || format!(
                        "opacity: {}; transform: translateX(-{}px)",
                        nav_opacity.get(),
                        nav_transform.get()
                    )
                >
                    <a
                        href="#home"
                        class=move || if active_nav.get() == "home" { "active" } else { "" }
                        on:click=move |_| set_active_nav.set("home".to_string())
                    >
                        <span class="prefix">"~/"</span>"home"
                    </a>
                    <a
                        href="#projects"
                        class=move || if active_nav.get() == "projects" { "active" } else { "" }
                        on:click=move |_| set_active_nav.set("projects".to_string())
                    >
                        <span class="prefix">"~/"</span>"projects"
                    </a>
                    <a
                        href="#blog"
                        class=move || if active_nav.get() == "blog" { "active" } else { "" }
                        on:click=move |_| set_active_nav.set("blog".to_string())
                    >
                        <span class="prefix">"~/"</span>"blog"
                    </a>
                    <a
                        href="#contact"
                        class=move || if active_nav.get() == "contact" { "active" } else { "" }
                        on:click=move |_| set_active_nav.set("contact".to_string())
                    >
                        <span class="prefix">"~/"</span>"contact"
                    </a>
                </nav>
                <div class="socials" style=move || format!("opacity: {}", socials_opacity.get())>
                    <a href="#">"[github]"</a>
                    <a href="#">"[linkedin]"</a>
                    <a href="#">"[twitter]"</a>
                </div>
            </div>
        </aside>
    }
}

/// Dynamic hacker scramble text runner with state lock
fn run_scramble(
    set_name: WriteSignal<String>,
    set_scrambling: WriteSignal<bool>,
    duration_ms: f64,
) {
    let window = web_sys::window().unwrap();
    let performance = window.performance().unwrap();
    let start = performance.now();
    
    set_scrambling.set(true);

    let cb = Closure::wrap(Box::new(move || {
        let elapsed = web_sys::window().unwrap().performance().unwrap().now() - start;
        let mut scramble = ScrambleText::new("Vidya Bodepudi", duration_ms);
        let text = scramble.text_at(elapsed);
        set_name.set(text);
    }) as Box<dyn Fn()>);

    let interval_id = window
        .set_interval_with_callback_and_timeout_and_arguments_0(
            cb.as_ref().unchecked_ref(),
            33, // 30 FPS updates
        )
        .unwrap();

    let window2 = window.clone();
    let cleanup = Closure::wrap(Box::new(move || {
        window2.clear_interval_with_handle(interval_id);
        set_scrambling.set(false);
        // Guarantee clean resolve to the final name string
        set_name.set("Vidya Bodepudi".to_string());
    }) as Box<dyn Fn()>);
    window
        .set_timeout_with_callback_and_timeout_and_arguments_0(
            cleanup.as_ref().unchecked_ref(),
            (duration_ms + 30.0) as i32,
        )
        .unwrap();
    cb.forget();
    cleanup.forget();
}
