use leptos::prelude::*;
use wasm_bindgen::prelude::*;

/// Contact section with interactive terminal connection animation
#[component]
pub fn Contact() -> impl IntoView {
    let (section_visible, set_section_visible) = signal(false);
    let (progress, set_progress) = signal(0.0_f64);
    let (connection_text, set_connection_text) = signal(String::new());
    let (email_opacity, set_email_opacity) = signal(0.0_f64);
    let (email_glow, set_email_glow) = signal(0.0_f64);
    let (pgp_opacity, set_pgp_opacity) = signal(0.0_f64);

    let section_ref = NodeRef::<leptos::html::Section>::new();

    // IntersectionObserver to trigger when scrolled into view
    Effect::new(move |_| {
        let Some(section) = section_ref.get() else { return };

        let cb = Closure::wrap(Box::new(move |entries: js_sys::Array, _observer: JsValue| {
            for i in 0..entries.length() {
                let entry: web_sys::IntersectionObserverEntry =
                    entries.get(i).unchecked_into();
                if entry.is_intersecting() {
                    set_section_visible.set(true);
                }
            }
        }) as Box<dyn Fn(js_sys::Array, JsValue)>);

        let mut options = web_sys::IntersectionObserverInit::new();
        options.set_threshold(&JsValue::from_f64(0.3));

        let observer =
            web_sys::IntersectionObserver::new_with_options(cb.as_ref().unchecked_ref(), &options)
                .unwrap();

        let el: &web_sys::Element = section.as_ref();
        observer.observe(el);
        cb.forget();
    });

    // Connection sequence animation
    Effect::new(move |_| {
        if !section_visible.get() {
            return;
        }

        let window = web_sys::window().unwrap();
        let performance = window.performance().unwrap();
        let start = performance.now();

        let connection_phases = vec![
            (0.0, "Initiating secure handshake..."),
            (800.0, "Verifying certificate chain..."),
            (1600.0, "Establishing encrypted tunnel..."),
            (2400.0, "Connection established. Send payload to:"),
        ];

        let cb = Closure::wrap(Box::new(move || {
            let elapsed = web_sys::window().unwrap().performance().unwrap().now() - start;

            // Progress bar animation (0-2400ms)
            let prog = (elapsed / 2400.0).min(1.0);
            // ease-in-out-cubic
            let eased_prog = if prog < 0.5 {
                4.0 * prog * prog * prog
            } else {
                1.0 - (-2.0 * prog + 2.0).powi(3) / 2.0
            };
            set_progress.set(eased_prog * 100.0);

            // Phase-based text updates
            let mut current_text = String::new();
            for (phase_time, phase_text) in &connection_phases {
                if elapsed > *phase_time {
                    current_text = phase_text.to_string();
                }
            }
            set_connection_text.set(current_text);

            // Email reveal at t=2800ms with spring glow
            if elapsed > 2800.0 {
                let local = elapsed - 2800.0;
                let t = (local / 400.0).min(1.0);
                let eased = 1.0 - (1.0 - t).powi(3);
                set_email_opacity.set(eased);

                // Pulsing glow effect (spring-like)
                if local < 2000.0 {
                    let glow = (local / 200.0).sin().abs() * (1.0 - local / 2000.0);
                    set_email_glow.set(glow);
                } else {
                    set_email_glow.set(0.0);
                }
            }

            // PGP key reveal at t=3200ms
            if elapsed > 3200.0 {
                let local = elapsed - 3200.0;
                let t = (local / 400.0).min(1.0);
                let eased = 1.0 - (1.0 - t).powi(3);
                set_pgp_opacity.set(eased);
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
                5000,
            )
            .unwrap();
        cb.forget();
        cleanup.forget();
    });

    view! {
        <section id="contact" class="section visible" node_ref=section_ref>
            <h2><span class="text-green">"#"</span>" Establish Connection"</h2>
            <div class="terminal-window">
                <div class="terminal-header">
                    <span class="btn red"></span>
                    <span class="btn yellow"></span>
                    <span class="btn green"></span>
                    <span class="title">"secure-channel:~"</span>
                </div>
                <div class="terminal-body contact-body">
                    // Progress bar
                    <div class="connection-progress">
                        <div
                            class="connection-progress-bar"
                            style=move || format!("width: {}%", progress.get())
                        ></div>
                    </div>
                    <p class="output">{move || connection_text.get()}</p>
                    <a
                        href="mailto:hello@example.com"
                        class="email-link"
                        style=move || format!(
                            "opacity: {}; box-shadow: 0 0 {}px rgba(0, 255, 65, {})",
                            email_opacity.get(),
                            15.0 + 20.0 * email_glow.get(),
                            0.3 + 0.5 * email_glow.get()
                        )
                    >
                        "hello@example.com"
                    </a>
                    <p
                        class="output mt-2"
                        style=move || format!("opacity: {}", pgp_opacity.get())
                    >
                        "PGP Key ID: "<span class="text-green">"0x0A1B2C3D4E5F6G7H"</span>
                    </p>
                </div>
            </div>
        </section>
    }
}
