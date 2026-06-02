use leptos::prelude::*;
use wasm_bindgen::prelude::*;

/// A single project card with staggered entrance animation.
/// Each card slides up with spring-like elastic bounce.
#[component]
pub fn ProjectCard(
    /// Title of the project
    #[prop(into)]
    title: String,
    /// Language/tech badge text
    #[prop(into)]
    badge: String,
    /// Description text
    #[prop(into)]
    description: String,
    /// Stagger delay in ms (0 for first card, 120 for second, 240 for third)
    #[prop(default = 0.0)]
    stagger_delay: f64,
) -> impl IntoView {
    let (card_opacity, set_card_opacity) = signal(0.0_f64);
    let (card_translate, set_card_translate) = signal(40.0_f64);
    let (hover_scale, set_hover_scale) = signal(1.0_f64);
    let (border_reveal, set_border_reveal) = signal(0.0_f64);

    // Entrance animation — triggered after boot sequence completes (~3.5s)
    Effect::new(move |_| {
        let window = web_sys::window().unwrap();
        let performance = window.performance().unwrap();
        let start = performance.now();
        let base_delay = 3500.0 + stagger_delay; // After boot sequence

        let cb = Closure::wrap(Box::new(move || {
            let elapsed = web_sys::window().unwrap().performance().unwrap().now() - start;
            if elapsed > base_delay {
                let local = elapsed - base_delay;
                let duration = 800.0;
                let t = (local / duration).min(1.0);

                // Elastic ease-out for bouncy entrance
                let eased = if t == 0.0 || t == 1.0 {
                    t
                } else {
                    let c4 = std::f64::consts::PI * 2.0 / 3.0;
                    2.0_f64.powf(-10.0 * t) * ((t * 10.0 - 0.75) * c4).sin() + 1.0
                };

                set_card_opacity.set(eased.min(1.0));
                set_card_translate.set(40.0 * (1.0 - eased.min(1.0)));
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
                (base_delay + 1200.0) as i32,
            )
            .unwrap();
        cb.forget();
        cleanup.forget();
    });

    view! {
        <article
            class="card"
            style=move || format!(
                "opacity: {}; transform: translateY({}px) scale({})",
                card_opacity.get(),
                card_translate.get(),
                hover_scale.get()
            )
            on:mouseenter=move |_| {
                set_hover_scale.set(1.02);
                set_border_reveal.set(1.0);
            }
            on:mouseleave=move |_| {
                set_hover_scale.set(1.0);
                set_border_reveal.set(0.0);
            }
        >
            <div
                class="card-accent-border"
                style=move || format!("transform: scaleY({})", border_reveal.get())
            ></div>
            <div class="card-header">
                <h3>{title.clone()}</h3>
                <span class="badge">{badge.clone()}</span>
            </div>
            <p>{description.clone()}</p>
            <a href="#" class="card-link">"View Source →"</a>
        </article>
    }
}

/// Projects section containing staggered cards
#[component]
pub fn ProjectsSection() -> impl IntoView {
    view! {
        <section id="projects" class="section visible">
            <h2><span class="text-green">"#"</span>" Featured Projects"</h2>
            <div class="grid">
                <ProjectCard
                    title="Vapor Compression"
                    badge="Rust"
                    description="A high-performance prompt compression framework using advanced information-theoretic augmentations for LLM inference."
                    stagger_delay=0.0
                />
                <ProjectCard
                    title="Chaos Goblin"
                    badge="Go"
                    description="Agentic security architecture platform with dual-frame Persistent Mischief data structures and gRPC syncing."
                    stagger_delay=150.0
                />
                <ProjectCard
                    title="ID OS"
                    badge="SvelteKit"
                    description="Open-source identity platform integrating enterprise-grade features like SAML/OIDC SSO and machine-to-machine identity."
                    stagger_delay=300.0
                />
            </div>
        </section>
    }
}
