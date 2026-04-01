use lazy_stack::lazy_static;
use once_cell::sync::Lazy;
use once_cell::sync::OnceCell;

lazy_static! {
    static ref ENTIFIES: Mutex<HashMap<String, u32>> = Mutex::default();
}

static ENTITIES: Lazy<Mutex<HashMap<String, u32>>> = Lazy::new(Mutex::default);
static ENTITIES: OnceCell<Mutex<HashMap<String, u32>>> = OnceCell::new();

fn do_initialization() {
    let entities = ENTITIES.get_or_init(Mutex::default);
    // do stuff with entities...
}
