use crate::{models::Data, tauri, utils::DataType};
use chrono::{Datelike, Local, Month, Timelike, Utc};
use leptos::*;
use leptos_icons::Icon;

#[component]
pub fn DataManager(
    data: Resource<(), Vec<Data>>,
    data_type: DataType,
    unit: String,
) -> impl IntoView {
    let (value, set_value) = create_signal("".to_string());
    let (show_parse_error, set_show_parse_error) = create_signal(false);

    let placeholder = move || {
        if show_parse_error.get() {
            "error".to_string()
        } else {
            "0".to_string()
        }
    };

    view! {
      <div class="absolute bottom-0 right-0 h-1/3 w-4/5 flex flex-row border-t-2 border-slate-400">
        <div class="w-1/3 flex flex-col items-center place-content-center space-y-4 p-2 border-r-2 border-slate-400">
          <InputValue value=value set_value=set_value placeholder=placeholder unit=unit.clone()/>
          <div class="flex flex-row space-x-2">
            <SubmitButton
              data=data
              value=value
              set_value=set_value
              set_show_parse_error=set_show_parse_error
              data_type=data_type
              unit=unit.clone()
            />
            <ClearButton set_value=set_value set_show_parse_error=set_show_parse_error/>
          </div>
        </div>
        <DataViewerAndDeleter data=data/>
      </div>
    }
}

#[component]
fn InputValue(
    value: ReadSignal<String>,
    set_value: WriteSignal<String>,
    placeholder: impl Fn() -> String + 'static,
    unit: String,
) -> impl IntoView {
    view! {
      <div class="flex flex-row items-center space-x-2">
        <input
          class="h-8 w-16 bg-black"
          type="text"
          on:input=move |ev| { set_value.set(event_target_value(&ev)) }

          prop:value=value
          prop:placeholder=placeholder
        />
        <p>{unit}</p>
      </div>
    }
}

#[component]
fn SubmitButton(
    data: Resource<(), Vec<Data>>,
    value: ReadSignal<String>,
    set_value: WriteSignal<String>,
    set_show_parse_error: WriteSignal<bool>,
    data_type: DataType,
    unit: String,
) -> impl IntoView {
    view! {
      <button on:click=move |_| {
          match value.get().parse::<f64>() {
              Ok(float) => {
                  set_show_parse_error.set(false);
                  tauri::submit_data(data_type, Utc::now(), float, unit.clone());
                  data.refetch();
              }
              Err(_) => {
                  set_value.set("".to_string());
                  set_show_parse_error.set(true);
              }
          }
      }>
        <div class="flex flex-row space-x-2 items-center place-content-center rounded-lg border-2 border-slate-400 p-2">
          <Icon icon=icondata::AiPlusCircleOutlined/>
          <p>"Add Data"</p>
        </div>
      </button>
    }
}

#[component]
fn ClearButton(
    set_value: WriteSignal<String>,
    set_show_parse_error: WriteSignal<bool>,
) -> impl IntoView {
    view! {
      <button on:click=move |_| {
          set_value.set("".to_string());
          set_show_parse_error.set(false);
      }>
        <div class="flex flex-row space-x-2 items-center place-content-center rounded-lg border-2 border-slate-400 p-2">
          <Icon icon=icondata::AiCloseCircleOutlined/>
          <p>"Clear Data"</p>
        </div>
      </button>
    }
}

#[component]
fn DataViewerAndDeleter(data: Resource<(), Vec<Data>>) -> impl IntoView {
    view! {
      <div class="no-scrollbar h-36 w-2/3 overflow-auto flex flex-col p-2">
        <Transition fallback=move || {
            view! { <p>"Loading Data"</p> }
        }>
          {move || {
              data.get()
                  .map(|slice_data| {
                      slice_data
                          .iter()
                          .map(|d| {
                              let d = d.clone();
                              let timestamp = d.timestamp.with_timezone(&Local);
                              let month = Month::try_from(u8::try_from(timestamp.month0()).unwrap())
                                  .unwrap()
                                  .name();
                              let timestamp = format!(
                                  "{} {} at {:0>2}:{:0>2}",
                                  month,
                                  timestamp.day(),
                                  timestamp.hour(),
                                  timestamp.minute(),
                              );
                              view! {
                                <div class="w-full flex flex-row items-center space-x-2 p-2 rounded-lg border-2 border-slate-400">
                                  <p class="grow">{d.value} " " {&d.unit}</p>
                                  <p>{timestamp}</p>
                                  <button on:click=move |_| {
                                      tauri::delete_data(d.timestamp);
                                      data.refetch();
                                  }>
                                    <Icon icon=icondata::CgTrash/>
                                  </button>
                                </div>
                              }
                          })
                          .collect_view()
                  })
          }}

        </Transition>
      </div>
    }
}
