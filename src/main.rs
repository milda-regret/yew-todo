use yew::prelude::*;

#[derive(PartialEq, Clone)]
struct Todo {
    due: String,
    title: String,
    content: String,
}

#[function_component(App)]
fn app() -> Html {
    let added_list = use_state_eq(|| Vec::new());
    let add_todo = {
        let list = added_list.clone();
        Callback::from(move |_| {
            let todo = Todo {
                due: String::from("2023-10-18T17:11"),
                title: String::from("Dinner"),
                content: String::from("with my parents"),
            };
            let mut vec = (*list).clone();
            vec.push(todo);
            list.set(vec)
        })
    };
    let test = added_list.iter().map(|todo| html! {
        <div>
            <p>{ "due: " } {todo.due.clone()}</p>
            <p><em>{ "Title: " } {todo.title.clone()}</em></p>
            <p>{ "content: " } {todo.content.clone()}</p>
            <button>{ "done" }</button>
        </div>
    });

    html! {
        <>
            <h1>{ "TODO" }</h1>
            <div class="container">
                <h2>{ "ADD" }</h2>
                <p>{ "due" }</p>
                <input type="datetime-local" />
                <p>{ "title" }</p>
                <input type="text" />
                <p>{ "content" }</p>
                <input type="text" /> <br />
                <button onclick={ add_todo }>{ "ADD" }</button>
            </div>
            <div class="container">
                <h2>{ "TODO" }</h2>
                { for test }
            </div>
            <div class="container">
                <h2>{ "DONE" }</h2>
                // { for }
            </div>
        </>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
