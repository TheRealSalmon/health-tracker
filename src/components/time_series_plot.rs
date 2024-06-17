use crate::models::Data;
use leptos::*;

#[component]
pub fn TimeSeriesPlot(data: Resource<(), Vec<Data>>) -> impl IntoView {
    let data_to_view = move |slice_data: &[Data]| {
        let timestamps: Vec<_> = slice_data.iter().map(|d| d.timestamp).collect();
        let values: Vec<_> = slice_data.iter().map(|d| d.value).collect();

        view! {
          <p>{format!("{:?}", timestamps)}</p>
          <p>{format!("{:?}", values)}</p>
        }
    };

    view! {
      <div class="grow">
        <Transition fallback=move || {
            view! { <p>"Loading Data"</p> }
        }>{move || { data.get().map(|data| data_to_view(&data)) }}</Transition>
      </div>
    }
}
