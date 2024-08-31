use home::HomeView;
use leptos::*;
use leptos_router::*;
use list::ListView;
use login::LoginView;
use order::OrderView;
use thaw::*;

mod home;
mod list;
mod login;
pub mod model;
mod order;

pub const API_PATH: &str = "http://localhost:3000/api";

#[derive(Copy, Clone)]
struct Context {
    login: RwSignal<bool>,
}

#[component]
fn App() -> impl IntoView {
    let theme = create_rw_signal(Theme::light());
    let login = create_rw_signal(false);

    provide_context(Context { login });

    view! {
        <Router>
            <ThemeProvider theme>
                <MessageProvider>
                    <Routes>
                        <Route path="/"           view=HomeView />
                        <Route path="/login"      view=LoginView />
                        <Route path="/orders/:id" view=OrderView />
                        <Route path="/orders"     view=ListView />
                        <Route path="*any" view=||view!{<h1>"Nie znaleziono strony"</h1>}/>
                    </Routes>
                </MessageProvider>
            </ThemeProvider>
        </Router>
    }
}

fn main() {
    console_error_panic_hook::set_once();
    println!("Hello, world!");
    mount_to_body(App);
}
