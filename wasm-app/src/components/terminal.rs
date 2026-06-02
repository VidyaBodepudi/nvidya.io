use leptos::prelude::*;

/// Reusable terminal window widget with header (red/yellow/green dots) and body
#[component]
pub fn Terminal(
    /// Title shown in the terminal header
    #[prop(into)]
    title: String,
    /// Content inside the terminal body
    children: Children,
) -> impl IntoView {
    let (hover_lift, set_hover_lift) = signal(0.0_f64);
    let (hover_glow, set_hover_glow) = signal(0.0_f64);

    view! {
        <div
            class="terminal-window"
            style=move || format!(
                "transform: translateY({}px); box-shadow: 0 15px 35px rgba(0,0,0,0.6), 0 0 {}px rgba(0,255,65,{})",
                -hover_lift.get(),
                20.0 * hover_glow.get(),
                0.05 * hover_glow.get()
            )
            on:mouseenter=move |_| {
                set_hover_lift.set(3.0);
                set_hover_glow.set(1.0);
            }
            on:mouseleave=move |_| {
                set_hover_lift.set(0.0);
                set_hover_glow.set(0.0);
            }
        >
            <div class="terminal-header">
                <span class="btn red"></span>
                <span class="btn yellow"></span>
                <span class="btn green"></span>
                <span class="title">{title}</span>
            </div>
            <div class="terminal-body">
                {children()}
            </div>
        </div>
    }
}
