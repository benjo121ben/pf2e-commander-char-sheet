use leptos::*;

#[derive(Clone, Default, Debug, Eq, PartialEq)] 
pub struct SimpleModalData {
    pub visible: bool, 
    pub title: String, 
    pub description: String
}

#[component]
pub fn SimpleModalView(data: RwSignal<SimpleModalData>) -> impl IntoView {
    view! {
        <Show when=move||data.get().visible>
            <div class="modal" on:click=move |_| data.update(|data| data.visible = false)>
                <div class="modal-content" on:click=move |_| {}>
                    <h1>{move|| data.get().title}</h1>
                    <hr style="margin-bottom:5px"/>
                    <p class="modal-description" inner_html={move|| data.get().description}/>
                </div>
            </div>
        </Show>
    }
}