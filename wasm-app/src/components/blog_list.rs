use leptos::prelude::*;
use wasm_bindgen::prelude::*;

/// Blog list with directional stagger — odd posts enter from left, even from right
#[component]
pub fn BlogList() -> impl IntoView {
    let posts = vec![
        ("2026-05-07", "Architecting AI-Native Version Control"),
        ("2026-05-05", "Security Tokenomics: Asymmetric Warfare"),
        ("2026-04-29", "Synthesizing the BNSI Research Paper"),
        ("2026-04-25", "Architecting Genomic Data Compression"),
    ];

    let (section_visible, set_section_visible) = signal(false);

    // Use IntersectionObserver to trigger entrance when scrolled into view
    let section_ref = NodeRef::<leptos::html::Section>::new();

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
        options.set_threshold(&JsValue::from_f64(0.2));

        let observer =
            web_sys::IntersectionObserver::new_with_options(cb.as_ref().unchecked_ref(), &options)
                .unwrap();

        let el: &web_sys::Element = section.as_ref();
        observer.observe(el);
        cb.forget();
    });

    view! {
        <section id="blog" class="section visible" node_ref=section_ref>
            <h2><span class="text-green">"#"</span>" Latest Logs"</h2>
            <div class="post-list">
                {posts
                    .into_iter()
                    .enumerate()
                    .map(|(i, (date, title))| {
                        view! {
                            <BlogPost
                                date=date.to_string()
                                title=title.to_string()
                                index=i
                                visible=section_visible
                            />
                        }
                    })
                    .collect::<Vec<_>>()}
            </div>
        </section>
    }
}

/// Individual blog post with directional entrance
#[component]
fn BlogPost(
    #[prop(into)] date: String,
    #[prop(into)] title: String,
    index: usize,
    visible: ReadSignal<bool>,
) -> impl IntoView {
    let (post_opacity, set_post_opacity) = signal(0.0_f64);
    let (post_translate_x, set_post_translate_x) = signal(
        if index % 2 == 0 { -30.0 } else { 30.0 },
    );
    let (hover_x, set_hover_x) = signal(0.0_f64);

    // Animate when section becomes visible
    Effect::new(move |_| {
        if !visible.get() {
            return;
        }

        let window = web_sys::window().unwrap();
        let performance = window.performance().unwrap();
        let start = performance.now();
        let delay = index as f64 * 100.0; // 100ms stagger between posts
        let direction = if index % 2 == 0 { -30.0 } else { 30.0 };

        let cb = Closure::wrap(Box::new(move || {
            let elapsed = web_sys::window().unwrap().performance().unwrap().now() - start;
            if elapsed > delay {
                let local = elapsed - delay;
                let duration = 600.0;
                let t = (local / duration).min(1.0);
                let eased = 1.0 - (1.0 - t).powi(3); // ease-out-cubic
                set_post_opacity.set(eased);
                set_post_translate_x.set(direction * (1.0 - eased));
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
                (delay + 800.0) as i32,
            )
            .unwrap();
        cb.forget();
        cleanup.forget();
    });

    view! {
        <a
            href="#"
            class="post-item"
            style=move || format!(
                "opacity: {}; transform: translateX({}px)",
                post_opacity.get(),
                post_translate_x.get() + hover_x.get()
            )
            on:mouseenter=move |_| set_hover_x.set(5.0)
            on:mouseleave=move |_| set_hover_x.set(0.0)
        >
            <span class="post-date">{date.clone()}</span>
            <span class="post-title">{title.clone()}</span>
        </a>
    }
}
