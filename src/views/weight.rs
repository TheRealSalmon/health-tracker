use crate::components::{DataManager, TimeSeriesPlot};
use crate::tauri;
use crate::utils::DataType;
use leptos::*;

#[component]
pub fn Weight() -> impl IntoView {
    let data = create_resource(
        || (),
        move |_| async move { tauri::get_data(DataType::Weight).await },
    );

    view! {
      <div class="h-full flex flex-col">
        <TimeSeriesPlot/>
        <DataManager data=data data_type=DataType::Weight unit="lbs".to_string()/>
      </div>
    }
}
