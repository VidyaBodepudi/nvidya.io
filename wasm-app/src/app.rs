use leptos::prelude::*;

use crate::components::sidebar::Sidebar;
use crate::components::hero::Hero;
use crate::components::project_card::ProjectsSection;
use crate::components::blog_list::BlogList;
use crate::components::contact::Contact;

/// Root application component — assembles all sections
#[component]
pub fn App() -> impl IntoView {
    view! {
        <Sidebar />
        <main class="content">
            <Hero />
            <ProjectsSection />
            <BlogList />
            <Contact />
        </main>
    }
}
