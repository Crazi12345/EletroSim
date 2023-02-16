use std::rc::Rc;
use yew::prelude::*;





fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    log::debug!("Apps is starting");
    yew::Renderer::<App>::new().render();
}

#[function_component(App)]
pub fn app() -> Html {
    let x_cor = use_state(|| 0);
    let y_cor = use_state(|| 40);
    let isdraggin = use_state(|| false);

    let onmousedown = {
        let isdraggin = isdraggin.clone();
        move |event: MouseEvent| {
            log::debug!("down");
            isdraggin.set(true);
                  }

    };



     let onmouseup = {
        let isdraggin = isdraggin.clone();
        move |event: MouseEvent| {
            log::debug!("up");

                isdraggin.set(false);



        }
    };

     let onmousemove = {
        let x_cor = x_cor.clone();
        let y_cor = y_cor.clone();
        let isdraggin = isdraggin.clone();
        move |_event: MouseEvent| {
            if *isdraggin {
                log::debug!("move");
                x_cor.set(_event.client_x());
                y_cor.set(_event.client_y());
            }
            }

    };

    html! {

        <div  >
            <h2 class={"heading"}>{"Hello, World!"}</h2>
            <button>{"hello"}</button>
        <div {onmouseup}{onmousemove}class={"colorChange"}>
            <svg  draggable="true" width="1000" height="1000">

        <circle onmousedown={onmousedown} class={"draggable"} cx={format!("{}",(*x_cor-10))} cy={format!("{}",(*y_cor-70))} r="40" stroke="green" stroke-width="1" fill="yellow" ></circle>
     //   <circle onmousedown={&onmousedown} class={"draggable"} cx={format!("{}",(*x_cor))} cy={format!("{}",(*y_cor))} r="40" stroke="green" stroke-width="1" fill="yellow" ></circle>

            </svg>
</div>

        </div>
    }
}

