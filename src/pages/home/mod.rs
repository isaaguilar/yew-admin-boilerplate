use crate::{
    layouts::{PageHeader, Section},
    store::FilterStore,
    util::util::{console_err as err, onclick_event_manager, RequestUtil},
    Route, API_ENDPOINT,
};
use gloo_storage::{LocalStorage, Storage};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::ops::Deref;
use web_sys::HtmlInputElement;
use yew::prelude::*;
use yew_router::prelude::*;
use yewdux::prelude::*;

#[derive(Serialize, Deserialize, Debug, Default)]
struct GenericResponse {
    status_info: StatusInfo,
    data: Option<Vec<Value>>,
}

#[derive(Serialize, Deserialize, Debug, Default)]
struct StatusInfo {
    status_code: u16,
    message: String,
}

#[derive(Serialize, Deserialize, Debug, Default)]
struct WorkflowData {
    name: String,
    namespace: String,
    cluster_name: String,
    state: String,
    uuid: String,
    current_generation: String,
    updated_at: String,
    created_at: String,
}

#[function_component(Home)]
pub fn home() -> Html {
    {
        // Redirect any route to Workflows page to /workflows path
        let navigator = use_navigator().unwrap();
        use_effect_with((), move |_| navigator.push(&Route::Workflows))
    }

    let navigator = use_navigator().unwrap();
    let location = use_location().unwrap();

    let location = location.clone();
    let query = match location.query::<Value>() {
        Ok(value) => Some(value),
        Err(e) => {
            err(&e);
            None
        }
    };
    let total_resources: UseStateHandle<Option<u64>> = use_state(|| None);
    let total_failed_resources: UseStateHandle<Option<u64>> = use_state(|| None);
    let workflows: UseStateHandle<Vec<WorkflowData>> = use_state(std::vec::Vec::new);
    let total_workflows_in_query: UseStateHandle<Option<u64>> = use_state(|| None);
    let fetch = use_state(|| false);
    let (filter_store, filter_dispatch) = use_store::<FilterStore>();
    let resource_name_filter = NodeRef::default();
    let cluster_filter = NodeRef::default();
    let namespace_filter: NodeRef = NodeRef::default();
    let input_elements_initialized = use_state(|| false);
    {
        let query = query.clone();
        use_effect_with((), move |_| {
            if let Some(value) = query.clone() {
                if let Some(value) = value.get("token") {
                    if let Some(token) = value.as_str() {
                        let local_storage = LocalStorage::raw();
                        let _ = local_storage.set("token", &token);
                    }
                }
            }
        });
    }
    {
        let fetch = fetch.clone();
        let filter_dispatch = filter_dispatch.clone();
        let input_elements_initialized = input_elements_initialized.clone();
        let query_value = match query.clone() {
            Some(value) => match value.get("resource_name") {
                Some(value) => match value.as_str() {
                    Some(s) => Some(s.to_string()),
                    None => None,
                },
                None => None,
            },
            None => None,
        };
        let node_ref = resource_name_filter.clone();
        let store_value = filter_store.resource_name.clone();
        use_effect_with(
            (input_elements_initialized, node_ref),
            move |(input_elements_initialized, node_ref)| {
                let filter_dispatch = filter_dispatch.clone();
                let fetch = fetch.clone();
                let node_ref = node_ref.clone();
                let input_elements_initialized = input_elements_initialized.clone();
                wasm_bindgen_futures::spawn_local(async move {
                    if *input_elements_initialized.deref() {
                        return;
                    }
                    let input_element = match node_ref.cast::<HtmlInputElement>() {
                        Some(elem) => elem,
                        None => return,
                    };

                    input_element.set_value(store_value.as_str());
                    if let Some(value) = query_value {
                        input_element.set_value(&value);
                        filter_dispatch.reduce_mut(|store| store.resource_name = value);
                        fetch.set(true);
                    }
                    input_elements_initialized.set(true);
                });
                || ()
            },
        );
    }
    {
        let fetch = fetch.clone();
        let filter_dispatch = filter_dispatch.clone();
        let input_elements_initialized = input_elements_initialized.clone();
        let query_value = match query.clone() {
            Some(value) => match value.get("cluster") {
                Some(value) => match value.as_str() {
                    Some(s) => Some(s.to_string()),
                    None => None,
                },
                None => None,
            },
            None => None,
        };
        let node_ref = cluster_filter.clone();
        let store_value = filter_store.cluster.clone();
        use_effect_with(
            (input_elements_initialized, node_ref),
            move |(input_elements_initialized, node_ref)| {
                let fetch = fetch.clone();
                let filter_dispatch = filter_dispatch.clone();
                let node_ref = node_ref.clone();
                let input_elements_initialized = input_elements_initialized.clone();
                wasm_bindgen_futures::spawn_local(async move {
                    if *input_elements_initialized.deref() {
                        return;
                    }
                    let input_element = match node_ref.cast::<HtmlInputElement>() {
                        Some(elem) => elem,
                        None => return,
                    };

                    input_element.set_value(store_value.as_str());
                    if let Some(value) = query_value {
                        input_element.set_value(&value);
                        filter_dispatch.reduce_mut(|store| store.cluster = value);
                        fetch.set(true);
                    }
                    input_elements_initialized.set(true);
                });
                || ()
            },
        );
    }
    {
        let fetch = fetch.clone();
        let filter_dispatch = filter_dispatch.clone();
        let input_elements_initialized = input_elements_initialized.clone();
        let query_value = match query.clone() {
            Some(value) => match value.get("namespace") {
                Some(value) => match value.as_str() {
                    Some(s) => Some(s.to_string()),
                    None => None,
                },
                None => None,
            },
            None => None,
        };
        let node_ref = namespace_filter.clone();
        let store_value = filter_store.namespace.clone();
        use_effect_with(
            (input_elements_initialized, node_ref),
            move |(input_elements_initialized, node_ref)| {
                let fetch = fetch.clone();
                let filter_dispatch = filter_dispatch.clone();
                let node_ref = node_ref.clone();
                let input_elements_initialized = input_elements_initialized.clone();
                wasm_bindgen_futures::spawn_local(async move {
                    if *input_elements_initialized.deref() {
                        return;
                    }
                    let input_element = match node_ref.cast::<HtmlInputElement>() {
                        Some(elem) => elem,
                        None => return,
                    };

                    input_element.set_value(store_value.as_str());
                    if let Some(value) = query_value {
                        input_element.set_value(&value);
                        filter_dispatch.reduce_mut(|store| store.namespace = value);
                        fetch.set(true);
                    }
                    input_elements_initialized.set(true);
                });
                || ()
            },
        );
    }
    {
        let total_resources = total_resources.clone();
        let total_failed_resources = total_failed_resources.clone();
        let workflows = workflows.clone();
        let total_workflows_in_query = total_workflows_in_query.clone();
        let filter_store = filter_store.clone();
        let filter_dispatch = filter_dispatch.clone();
        let fetch = fetch.clone();
        use_effect_with((), move |_| {
            wasm_bindgen_futures::spawn_local(async move {
                let url = &format!("{}/api/v1/metrics/total/resources", API_ENDPOINT);
                let fetched_data = match RequestUtil::new(url.to_string()).get().await {
                    Ok(response) => match response.json::<GenericResponse>(vec![200]).await {
                        Ok(data) => data,
                        Err(e) => return err(&e),
                    },
                    Err(e) => return err(&e),
                };

                if let Some(data) = fetched_data.data {
                    if let Some(t) = data[0].as_number() {
                        total_resources.set(t.as_u64());
                    }
                }
            });

            wasm_bindgen_futures::spawn_local(async move {
                let url = &format!("{}/api/v1/metrics/total/failed-resources", API_ENDPOINT);
                let fetched_data = match RequestUtil::new(url.to_string()).get().await {
                    Ok(response) => match response.json::<GenericResponse>(vec![200]).await {
                        Ok(data) => data,
                        Err(e) => return err(&e),
                    },
                    Err(e) => return err(&e),
                };
                if let Some(data) = fetched_data.data {
                    if let Some(t) = data[0].as_number() {
                        total_failed_resources.set(t.as_u64());
                    }
                }
            });
            || ()
        });

        use_effect_with(*fetch, move |_| {
            wasm_bindgen_futures::spawn_local(async move {
                let any_query = format!(
                    "name={} cluster={} namespace={}",
                    filter_store.resource_name, filter_store.cluster, filter_store.namespace
                );

                let url = &format!(
                    "{}/api/v1/workflows?limit=25&offset={}&matchAny={}",
                    API_ENDPOINT, filter_store.offset, any_query
                );

                let fetched_data = match RequestUtil::new(url.to_string()).get().await {
                    Ok(response) => match response.json::<GenericResponse>(vec![200]).await {
                        Ok(data) => data,
                        Err(e) => return err(&e),
                    },
                    Err(e) => return err(&e),
                };
                let data = if let Some(data) = fetched_data.data {
                    data.iter()
                        .map(|value| serde_json::from_value(value.clone()).unwrap())
                        .collect::<Vec<WorkflowData>>()
                } else {
                    vec![]
                };

                workflows.set(data);

                let url = &format!(
                    "{}/api/v1/metrics/total/resources?matchAny={}",
                    API_ENDPOINT, any_query
                );
                let fetched_data = match RequestUtil::new(url.to_string()).get().await {
                    Ok(response) => match response.json::<GenericResponse>(vec![200]).await {
                        Ok(data) => data,
                        Err(e) => return err(&e),
                    },
                    Err(e) => return err(&e),
                };

                if let Some(data) = fetched_data.data {
                    if let Some(t) = data[0].as_number() {
                        total_workflows_in_query.set(t.as_u64());
                        if let Some(total) = t.as_u64() {
                            if filter_store.offset > total {
                                let mut pages = (total as f64 / 25.0).ceil() as u64;
                                pages = pages.saturating_sub(1);

                                filter_dispatch.reduce_mut(|store| store.offset = pages * 25);
                            }
                        }
                    }
                }

                fetch.set(false);
            });
            || ()
        });
    }

    let calculated_pages = match total_workflows_in_query.deref() {
        Some(t) => (*t as f64 / 25.0).ceil() as u64,
        None => 0,
    };
    let calculated_current_page = (1.0 + filter_store.clone().offset as f64 / 25.0).floor() as u64;

    let next_page = {
        let fetch = fetch.clone();
        filter_dispatch.reduce_mut_callback_with(move |store, _| {
            let total_workflows_in_query = total_workflows_in_query.clone();
            if store.offset + 25 > total_workflows_in_query.deref().unwrap_or(0) {
                return;
            }
            store.offset += 25;
            fetch.set(true);
        })
    };

    let back_page = {
        let fetch = fetch.clone();
        filter_dispatch.reduce_mut_callback_with(move |store, _| {
            if store.offset == 0 {
                return;
            } else if store.offset > 25 {
                store.offset -= 25;
            } else {
                store.offset = 0;
            }
            fetch.set(true);
        })
    };

    let filter_plus = {
        let navigator = navigator.clone();
        let location = location.clone();

        let fetch = fetch.clone();
        filter_dispatch.reduce_mut_callback_with(move |store, e: KeyboardEvent| {
            let filter_string = e.target_unchecked_into::<HtmlInputElement>().value();
            let mut query = location.query::<Value>().unwrap_or(json!({}));

            match e.target_unchecked_into::<HtmlInputElement>().id().as_str() {
                "resource" => {
                    store.resource_name = filter_string.clone();
                    query
                        .as_object_mut()
                        .unwrap()
                        .insert(String::from("resource_name"), json!(filter_string.clone()));
                    let _ = navigator.replace_with_query(&Route::Workflows, &query);
                }
                "cluster" => {
                    store.cluster = filter_string.clone();
                    query
                        .as_object_mut()
                        .unwrap()
                        .insert(String::from("cluster"), json!(filter_string.clone()));
                    let _ = navigator.replace_with_query(&Route::Workflows, &query);
                }
                "namespace" => {
                    store.namespace = filter_string.clone();
                    query
                        .as_object_mut()
                        .unwrap()
                        .insert(String::from("namespace"), json!(filter_string.clone()));
                    let _ = navigator.replace_with_query(&Route::Workflows, &query);
                }
                _ => {}
            }

            fetch.set(true);
        })
    };

    let onchange = {
        let navigator = navigator.clone();
        let location = location.clone();

        let fetch = fetch.clone();
        filter_dispatch.reduce_mut_callback_with(move |store, e: Event| {
            let filter_string = e.target_unchecked_into::<HtmlInputElement>().value();
            let mut query = location.query::<Value>().unwrap_or(json!({}));

            match e.target_unchecked_into::<HtmlInputElement>().id().as_str() {
                "resource" => {
                    store.resource_name = filter_string.clone();
                    query
                        .as_object_mut()
                        .unwrap()
                        .insert(String::from("resource_name"), json!(filter_string.clone()));
                    let _ = navigator.replace_with_query(&Route::Workflows, &query);
                }
                "cluster" => {
                    store.cluster = filter_string.clone();
                    query
                        .as_object_mut()
                        .unwrap()
                        .insert(String::from("cluster"), json!(filter_string.clone()));
                    let _ = navigator.replace_with_query(&Route::Workflows, &query);
                }
                "namespace" => {
                    store.namespace = filter_string.clone();
                    query
                        .as_object_mut()
                        .unwrap()
                        .insert(String::from("namespace"), json!(filter_string.clone()));
                    let _ = navigator.replace_with_query(&Route::Workflows, &query);
                }
                _ => {}
            }

            fetch.set(true);
        })
    };

    let workflow_rows = {
        workflows
            .iter()
            .map(|value| {
                html! {
                    <tr>
                        <td>
                            <Link<Route> to={Route::Workflows{}}>
                                    {&value.name}
                            </Link<Route>>
                        </td>
                        <td>
                            {&value.cluster_name}
                        </td>
                        <td>
                            {&value.namespace}
                        </td>
                        <td>
                            {&value.updated_at}
                        </td>
                        <td>
                            {&value.state}
                        </td>
                    </tr>
                }
            })
            .collect::<Vec<_>>()
    };

    let badges = html! {
        <>
            <div class="row row-cols-auto">
                <div>
                    <span class="badge rounded-pill text-white" style="background-color: #6f42c1">{"Total="}{total_resources.unwrap_or(0).to_string()}</span>
                </div>
                <div>
                    <span class="badge rounded-pill text-bg-danger">{"Failed="}{total_failed_resources.unwrap_or(0).to_string()}</span>
                </div>
            </div>
        </>
    };

    let paginator = html! {
        <>
        <div class="mb-2">
            <button class="btn btn-primary" onclick={onclick_event_manager(vec![back_page], filter_store.clone().offset)}>{"Back "}</button>
            <span>{format!(" Page {} of {} ", calculated_current_page, calculated_pages)}</span>
            <button class="btn btn-primary" onclick={onclick_event_manager(vec![next_page], filter_store.clone().offset)}>{"Next"}</button>
        </div>
        </>
    };
    html! {
        <>
            <PageHeader title="Workflows" raw_subtitle={badges} />
                <Section raw_title={html!{<h5>{"Data"}</h5>}}>
                    <Section raw_title={html!{<h5>{"Filters"}</h5>}}>
                    <div>
                        <div class="input-group mb-3">
                            <span class="input-group-text" id="inputGroup-sizing-default">{"Resource Name"}</span>
                            <input ref={resource_name_filter} id="resource" onchange={onchange.clone()} onkeyup={filter_plus.clone()} type="text" class="form-control" aria-label="Resource name filter" aria-describedby="inputGroup-sizing-default" />
                        </div>
                        <div class="input-group mb-3">
                            <span class="input-group-text" id="inputGroup-sizing-default">{"Cluster"}</span>
                            <input ref={cluster_filter} id="cluster" onchange={onchange.clone()} onkeyup={filter_plus.clone()} type="text" class="form-control" aria-label="Cluster filter" aria-describedby="inputGroup-sizing-default" />
                        </div>
                        <div class="input-group mb-3">
                            <span class="input-group-text" id="inputGroup-sizing-default">{"Namespace"}</span>
                            <input ref={namespace_filter} id="namespace" onchange={onchange.clone()} onkeyup={filter_plus.clone()} type="text" class="form-control" aria-label="Namespace filter" aria-describedby="inputGroup-sizing-default" />
                        </div>
                    </div>
                </Section>
                    <div class="row d-flex flex-row-reverse row-cols-auto">
                        {paginator.clone()}
                    </div>
                    <table class="table table-hover">
                        <thead>
                            <tr>
                            <th scope="col">{"Resource Name"}</th>
                            <th scope="col">{"Cluster"}</th>
                            <th scope="col">{"Namespace"}</th>
                            <th scope="col">{"Updated At"}</th>
                            <th scope="col">{"Status"}</th>
                            </tr>
                        </thead>
                        <tbody>
                            {workflow_rows}
                        </tbody>
                    </table>
                    <div>
                        {paginator.clone()}
                    </div>
                </Section>
        </>
    }
}
