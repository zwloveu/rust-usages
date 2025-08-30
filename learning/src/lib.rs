use std::{
    collections::HashMap,
    sync::{Mutex, MutexGuard, OnceLock},
};

mod terminal_guard;
pub use terminal_guard::{Key, read_key};

mod module01_smart_pointers;

static GLOBAL_REGISTRY: OnceLock<Mutex<HashMap<&'static str, fn()>>> = OnceLock::new();

pub trait FeatureRegistry {
    fn get_features(&self) -> HashMap<&'static str, fn()>;
}

pub fn init_features() {
    let registry: &Mutex<HashMap<&'static str, fn()>> =
        GLOBAL_REGISTRY.get_or_init(|| Mutex::new(HashMap::new()));

    let mut features: MutexGuard<'_, HashMap<&'static str, fn()>> = registry.lock().unwrap();

    features.extend(
        &module01_smart_pointers::BoxPointerModuleFeatureRegister::default().get_features(),
    );
}

pub fn get_all_features() -> HashMap<&'static str, fn()> {
    let registry: &Mutex<HashMap<&'static str, fn()>> =
        GLOBAL_REGISTRY.get_or_init(|| Mutex::new(HashMap::new()));

    registry.lock().unwrap().clone()
}
