use leptos::*;

#[component]
pub fn Home() -> impl IntoView {
    view! {
      <div class="flex flex-col items-center space-y-2 p-2 text-center">
        <p class="text-2xl">"Welcome"</p>
        <p class="w-5/6">
          "
          This is health-tracker, a personal project written by Samuel Mun.
          It allows users to log their health data and plot it over time.
          "
        </p>
        <p class="w-5/6">
          "
          This project is written in Rust using Leptos for the frontend and Tauri to build the desktop app.
          "
        </p>
      </div>
    }
}
