use crate::models::Data;
use leptos::html::Div;
use leptos::*;
use leptos_use::{use_element_size, UseElementSizeReturn};

#[component]
pub fn TimeSeriesPlot(data: Resource<(), Vec<Data>>) -> impl IntoView {
    let el = create_node_ref::<Div>();
    let UseElementSizeReturn { width, height } = use_element_size(el);
    view! {
      <div class="h-2/3" node_ref=el>
        <Transition fallback=move || {
            view! { <p>"Loading Data"</p> }
        }>
          {move || {
              data.get()
                  .map(|slice_data| {
                      let first = slice_data.first().unwrap().clone();
                      let data: Vec<Data> = slice_data
                          .iter()
                          .filter(|d| {
                              first.timestamp.signed_duration_since(d.timestamp).num_days() < 7
                          })
                          .cloned()
                          .collect();

                      view! {
                        <svg class="h-full w-full">
                          <line x1=10 y1=10 x2=10 y2=move || height.get() - 10.0 style="stroke-width:2;stroke:red;"/>
                        </svg>
                      }
                  })
          }}

        </Transition>
      </div>
    }
}
