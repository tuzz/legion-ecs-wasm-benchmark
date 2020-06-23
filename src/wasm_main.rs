use wasm_bindgen::prelude::*;
use legion::prelude::*;
use web_sys::window;

struct Foo;
struct Bar;

#[wasm_bindgen(start)]
pub fn main() {
    let user_agent = window().unwrap().navigator().user_agent().unwrap();
    write_to_html(&format!("Benchmarking {}", user_agent));
    write_to_html("-----------------------------------------------------");

    for power in 16..22 {
        let number_of_entities = 2_u32.pow(power);

        let time1 = now();
        let universe = Universe::new();
        let mut world = universe.create_world();
        world.insert((), (0..number_of_entities).map(|_| (Foo,)));

        let time2 = now();
        let query = Read::<Foo>::query();
        let entities = query.iter_entities(&world).map(|(e, _)| e).collect::<Vec<_>>();

        let time3 = now();
        for entity in entities {
            world.add_component(entity, Bar).unwrap();
        }

        write_to_html(&format!("Number of entities: {}", number_of_entities));
        write_to_html(&format!("Creating entities took {} ms", time2 - time1));
        write_to_html(&format!("Adding components took {} ms", time3 - time2));
        write_to_html("-----------------------------------------------------");
    }
}

fn write_to_html(s: &str) {
    let window = window().unwrap();
    let document = window.document().unwrap();
    let body = document.body().unwrap();
    let p = document.create_element("p").unwrap();

    p.set_inner_html(s);
    body.append_child(&p).unwrap();
}

fn now() -> f64 {
    window().unwrap().performance().unwrap().now()
}
