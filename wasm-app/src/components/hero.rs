use leptos::prelude::*;
use wasm_bindgen::prelude::*;

/// Hero section with typing effect and SVG circuit line-draw animation
#[component]
pub fn Hero() -> impl IntoView {
    let (hero_opacity, set_hero_opacity) = signal(0.0_f64);
    let (hero_translate, set_hero_translate) = signal(30.0_f64);
    let (typed_text, set_typed_text) = signal(String::new());
    let (cursor_visible, set_cursor_visible) = signal(true);
    let (svg_offset, set_svg_offset) = signal(1000.0_f64);
    let (terminal_opacity, set_terminal_opacity) = signal(0.0_f64);
    let (terminal_translate, set_terminal_translate) = signal(20.0_f64);

    // Orchestrated entrance
    Effect::new(move |_| {
        let window = web_sys::window().unwrap();
        let performance = window.performance().unwrap();

        // Hero section slides up at t=2000ms
        {
            let start = performance.now();
            let cb = Closure::wrap(Box::new(move || {
                let elapsed = web_sys::window().unwrap().performance().unwrap().now() - start;
                let delay = 2000.0;
                let duration = 800.0;
                if elapsed > delay {
                    let t = ((elapsed - delay) / duration).min(1.0);
                    let eased = 1.0 - (1.0 - t).powi(3);
                    set_hero_opacity.set(eased);
                    set_hero_translate.set(30.0 * (1.0 - eased));
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
                    3500,
                )
                .unwrap();
            cb.forget();
            cleanup.forget();
        }

        // SVG circuit line-draw at t=2200ms
        {
            let start = performance.now();
            let cb = Closure::wrap(Box::new(move || {
                let elapsed = web_sys::window().unwrap().performance().unwrap().now() - start;
                let delay = 2200.0;
                let duration = 1200.0;
                if elapsed > delay {
                    let t = ((elapsed - delay) / duration).min(1.0);
                    // ease-in-out-cubic
                    let eased = if t < 0.5 {
                        4.0 * t * t * t
                    } else {
                        1.0 - (-2.0 * t + 2.0).powi(3) / 2.0
                    };
                    set_svg_offset.set(1000.0 * (1.0 - eased));
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
                    4000,
                )
                .unwrap();
            cb.forget();
            cleanup.forget();
        }

        // Typing effect at t=2500ms
        {
            let hero_text = "Initializing system...";
            let chars: Vec<char> = hero_text.chars().collect();
            let len = chars.len();
            let start = performance.now();

            let cb = Closure::wrap(Box::new(move || {
                let elapsed = web_sys::window().unwrap().performance().unwrap().now() - start;
                let delay = 2500.0;
                if elapsed > delay {
                    let local = elapsed - delay;
                    let char_interval = 100.0; // ms per character
                    let idx = (local / char_interval).floor() as usize;
                    if idx <= len {
                        let text: String = chars[..idx.min(len)].iter().collect();
                        set_typed_text.set(text);
                    }
                }
            }) as Box<dyn Fn()>);

            let interval_id = window
                .set_interval_with_callback_and_timeout_and_arguments_0(
                    cb.as_ref().unchecked_ref(),
                    50,
                )
                .unwrap();

            let window2 = window.clone();
            let cleanup = Closure::wrap(Box::new(move || {
                window2.clear_interval_with_handle(interval_id);
            }) as Box<dyn Fn()>);
            window
                .set_timeout_with_callback_and_timeout_and_arguments_0(
                    cleanup.as_ref().unchecked_ref(),
                    5500,
                )
                .unwrap();
            cb.forget();
            cleanup.forget();
        }

        // Terminal window entrance at t=3000ms
        {
            let start = performance.now();
            let cb = Closure::wrap(Box::new(move || {
                let elapsed = web_sys::window().unwrap().performance().unwrap().now() - start;
                let delay = 3000.0;
                let duration = 600.0;
                if elapsed > delay {
                    let t = ((elapsed - delay) / duration).min(1.0);
                    let eased = 1.0 - (1.0 - t).powi(3);
                    set_terminal_opacity.set(eased);
                    set_terminal_translate.set(20.0 * (1.0 - eased));
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
                    4000,
                )
                .unwrap();
            cb.forget();
            cleanup.forget();
        }

        // Cursor blink
        {
            let cb = Closure::wrap(Box::new(move || {
                set_cursor_visible.update(|v| *v = !*v);
            }) as Box<dyn Fn()>);
            window
                .set_interval_with_callback_and_timeout_and_arguments_0(
                    cb.as_ref().unchecked_ref(),
                    530,
                )
                .unwrap();
            cb.forget();
        }
    });

    view! {
        <section
            id="home"
            class="section"
            style=move || format!(
                "opacity: {}; transform: translateY({}px)",
                hero_opacity.get(),
                hero_translate.get()
            )
        >
            <div class="hero">
                // SVG circuit pattern behind the hero text
                <svg class="circuit-svg" viewBox="0 0 800 100" xmlns="http://www.w3.org/2000/svg">
                    <path
                        class="circuit-path"
                        d="M 10,50 L 160,50 L 160,25 L 400,25 L 400,50 L 600,50 L 600,75 L 790,75 M 400,50 L 400,75 L 160,75 M 600,50 L 790,50"
                        style=move || format!(
                            "stroke-dasharray: 1000; stroke-dashoffset: {}",
                            svg_offset.get()
                        )
                    />
                </svg>
                <h2 class="typing-effect">
                    {move || typed_text.get()}
                    <span
                        class="cursor"
                        style=move || if cursor_visible.get() { "opacity: 1" } else { "opacity: 0" }
                    >"_"</span>
                </h2>
                <p class="subtitle">
                    "Welcome to my digital garden. I build secure systems and explore the depths of cyberspace."
                </p>
            </div>

            <div
                class="terminal-window"
                style=move || format!(
                    "opacity: {}; transform: translateY({}px)",
                    terminal_opacity.get(),
                    terminal_translate.get()
                )
            >
                <div class="terminal-header">
                    <span class="btn red"></span>
                    <span class="btn yellow"></span>
                    <span class="btn green"></span>
                    <span class="title">"guest@hacker-blog:~"</span>
                </div>
                <div class="terminal-body">
                    <p><span class="prompt">"guest@hacker-blog:~$"</span>" cat whoami.txt"</p>
                    <p class="output">
                        "I am a passionate software developer focusing on scalable web applications and cybersecurity. When I'm not coding, you can find me reverse engineering software or playing CTFs."
                    </p>
                    <p><span class="prompt">"guest@hacker-blog:~$"</span>" ./show_skills.sh"</p>
                    <p class="output text-green">
                        "> Rust, Go, Python"<br/>
                        "> Penetration Testing, Cryptography"<br/>
                        "> Linux Systems, Networking"
                    </p>
                    <p>
                        <span class="prompt">"guest@hacker-blog:~$"</span>
                        " "
                        <span
                            class="cursor"
                            style=move || if cursor_visible.get() { "opacity: 1" } else { "opacity: 0" }
                        >"_"</span>
                    </p>
                </div>
            </div>
        </section>
    }
}
