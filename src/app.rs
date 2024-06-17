use crate::components::*;
use crate::views::*;
use leptos::*;
use leptos_router::{Route, Router, Routes};

#[component]
pub fn App() -> impl IntoView {
    view! {
      <Router>
        <main class="flex flex-row text-slate-50">
          <NavBar/>
          <div class="w-4/5">
            <Routes>
              <Route path="/" view=Home/>
              <Route path="/weight" view=Weight/>
              <Route path="/temperature" view=Temperature/>
            </Routes>
          </div>
        </main>
      </Router>
    }
}
