//
// things to add
// 1. death alerts
// 2. custom gamespeed
// 3. resizable game board
// 4. reduce binary size
//
//

use std::collections::{vec_deque, VecDeque};

use ev::KeyboardEvent;
use leptos::*;
use leptos_dom::logging::console_log;
use rand::random;
use wasm_bindgen::{closure::Closure, JsCast};
const EMPTY_CELL: &str = "m-px w-12 h-12 rounded-md bg-slate-700";
const SNAKE_CELL: &str = "m-px w-12 h-12 rounded-md bg-emerald-500";
const FOOD_CELL: &str = "m-px w-12 h-12 rounded-sm bg-cyan-500";
const NUM_ROWS: u32 = 5;
const NUM_COLS: u32 = 5;

const STARTING_BODY: [(i32, i32); 3] = [(1, 1), (2, 1), (3, 1)];

enum Direction {
    Up(),
    Down(),
    Left(),
    Right(),
}

struct SnakeState {
    dir: Direction,
    body: VecDeque<(i32, i32)>,
    food: (i32, i32),
}

impl SnakeState {
    fn time_step(&mut self) {
        fn wrap_x(x: i32) -> i32 {
            if x < 0 {
                NUM_COLS as i32 + x
            } else if x >= NUM_COLS as i32 {
                x - NUM_COLS as i32
            } else {
                x
            }
        }
        fn wrap_y(y: i32) -> i32 {
            if y < 0 {
                NUM_ROWS as i32 + y
            } else if y >= NUM_ROWS as i32 {
                y - NUM_ROWS as i32
            } else {
                y
            }
        }
        let (new_x, new_y) = match (&self.dir, &self.body[self.body.len() - 1]) {
            (Direction::Up(), (x, y)) => (wrap_x(*x), wrap_y(*y - 1)),

            (Direction::Down(), (x, y)) => (wrap_x(*x), wrap_y(*y + 1)),

            (Direction::Left(), (x, y)) => (wrap_x(*x - 1), wrap_y(*y)),

            (Direction::Right(), (x, y)) => (wrap_x(*x + 1), wrap_y(*y)),
        };

        if self.body.len() == (NUM_ROWS * NUM_COLS) as usize {
            // game won
            console_log("YOU WON!!!");
            todo!();
        }

        if self.body.contains(&(new_x, new_y)) {
            // endgame cuz dead
            console_log("YOU DIED!!!");
            todo!();
        }

        if self.food == (new_x, new_y) {
            // increment length
            self.body.push_front((-1, -1));

            // add new food
            let mut food_ind = (
                (random::<u32>() % NUM_COLS) as i32,
                (random::<u32>() % NUM_ROWS) as i32,
            );
            while self.body.contains(&food_ind) {
                food_ind = (
                    (random::<u32>() % NUM_COLS) as i32,
                    (random::<u32>() % NUM_ROWS) as i32,
                );
            }
            self.food = food_ind;
            document()
                .get_element_by_id(format!("({},{})", food_ind.0, food_ind.1).as_str())
                .expect("ele should have loaded by now")
                .set_class_name(FOOD_CELL);
        }

        // add new head
        self.body.push_back((new_x, new_y));
        document()
            .get_element_by_id(format!("({new_x},{new_y})").as_str())
            .expect("ele should have loaded by now")
            .set_class_name(SNAKE_CELL);

        // remove old tail
        let (expired_x, expired_y) = self.body.pop_front().expect("Snake should never be empty");
        if expired_x >= 0 && expired_y >= 0 {
            document()
                .get_element_by_id(format!("({expired_x},{expired_y})").as_str())
                .expect("ele should have loaded by now")
                .set_class_name(EMPTY_CELL);
        }
    }
}

static mut STATE: SnakeState = SnakeState {
    dir: Direction::Right(),
    body: vec_deque::VecDeque::new(),
    food: (0, 0),
};

static mut FRAME: u64 = 0;
fn game_loop() {
    unsafe {
        if FRAME % 75 == 0 {
            STATE.time_step();
        }

        // update state
        FRAME += 1;

        request_animation_frame(game_loop);
    }
}

fn snake_keypress(key_event: KeyboardEvent) {
    let x = key_event.key().to_lowercase();
    match x.as_str() {
        "w" | "uparrow" => {
            unsafe { STATE.dir = Direction::Up() };
        }
        "s" | "downarrow" => {
            unsafe { STATE.dir = Direction::Down() };
        }
        "a" | "leftarrow" => {
            unsafe { STATE.dir = Direction::Left() };
        }
        "d" | "rightarrow" => {
            unsafe { STATE.dir = Direction::Right() };
        }
        _ => (),
    }
}

#[component]
pub fn Snake() -> impl IntoView {
    // init snake state
    spawn_local(async move {
        for (sx, sy) in STARTING_BODY {
            unsafe {
                STATE.body.push_back((sx, sy));
            }
            document()
                .get_element_by_id(format!("({sx},{sy})").as_str())
                .expect("ele should have loaded by now")
                .set_class_name(SNAKE_CELL);
        }
        unsafe {
            let mut food_ind = (
                (random::<u32>() % NUM_COLS) as i32,
                (random::<u32>() % NUM_ROWS) as i32,
            );
            while STATE.body.contains(&food_ind) {
                food_ind = (
                    (random::<u32>() % NUM_COLS) as i32,
                    (random::<u32>() % NUM_ROWS) as i32,
                );
            }
            STATE.food = food_ind;
            document()
                .get_element_by_id(format!("({},{})", food_ind.0, food_ind.1).as_str())
                .expect("ele should have loaded by now")
                .set_class_name(FOOD_CELL);
        }
    });

    // enable keyboard
    let key_closure: Closure<dyn FnMut(KeyboardEvent)> =
        wasm_bindgen::closure::Closure::new(snake_keypress);
    document()
        .add_event_listener_with_callback("keydown", key_closure.as_ref().unchecked_ref())
        .expect("could not add keyboard listener");
    key_closure.forget();

    let grid = (0..NUM_ROWS)
        .map(|y| {
            view! {
                <div class="flex justify-center">
                    {(0..NUM_COLS).map(|x| {
                        view! {
                            <div id={format!("({x},{y})")} class=EMPTY_CELL></div>
                        }
                    }).collect_view()}
                </div>
            }
        })
        .collect_view();
    request_animation_frame(game_loop);

    view! {
        <div class="flex flex-col my-20">
            { grid }
        </div>
    }
}