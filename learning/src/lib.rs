use std::{
    collections::HashMap,
    sync::{Mutex, MutexGuard, OnceLock},
};

mod module01_smart_pointers;

static GLOBAL_REGISTRY: OnceLock<Mutex<HashMap<&'static str, fn()>>> = OnceLock::new();

pub trait FeatureRegistry {
    fn get_features(&self) -> HashMap<&'static str, fn()>;
}

pub fn init_features() -> Result<(), Box<dyn std::error::Error>> {
    let registry: &Mutex<HashMap<&'static str, fn()>> =
        GLOBAL_REGISTRY.get_or_init(|| Mutex::new(HashMap::new()));

    let mut features: MutexGuard<'_, HashMap<&'static str, fn()>> = registry.lock()?;

    features.extend(
        &module01_smart_pointers::BoxPointerModuleFeatureRegister::default().get_features(),
    );

    Ok(())
}

pub fn get_all_features()
-> Result<MutexGuard<'static, HashMap<&'static str, fn()>>, Box<dyn std::error::Error>> {
    let registry: &Mutex<HashMap<&'static str, fn()>> =
        GLOBAL_REGISTRY.get_or_init(|| Mutex::new(HashMap::new()));

    Ok(registry.lock()?)
}
