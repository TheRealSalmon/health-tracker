use leptos::*;
use leptos_icons::*;

#[component]
pub fn NavBar() -> impl IntoView {
    view! {
      <div class="h-screen w-1/5 flex flex-col flex-nowrap items-center space-y-2 p-2 border-r-2 border-slate-400">
        <a class="w-full" href="/">
          <div class="h-12 flex space-x-2 pl-2 items-center rounded-lg border-2 border-slate-400">
            <Icon icon=icondata::ChHome/>
            <p>"Home"</p>
          </div>
        </a>
        <a class="w-full" href="/weight">
          <div class="h-12 flex space-x-2 pl-2 items-center rounded-lg border-2 border-slate-400">
            <Icon icon=icondata::BsPersonArmsUp/>
            <p>"Weight"</p>
          </div>
        </a>
        <a class="w-full" href="/temperature">
          <div class="h-12 flex space-x-2 pl-2 items-center rounded-lg border-2 border-slate-400">
            <Icon icon=icondata::TbTemperature/>
            <p>"Temperature"</p>
          </div>
        </a>
      </div>
    }
}
