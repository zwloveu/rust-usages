use std::{
    collections::HashMap,
    sync::{Mutex, MutexGuard, OnceLock},
};

mod module01_mutability_rules;
mod module02_data_structures;
mod module09_smart_pointers;

static GLOBAL_REGISTRY: OnceLock<Mutex<HashMap<&'static str, fn()>>> = OnceLock::new();

pub trait FeatureRegistry {
    fn get_features(&self) -> HashMap<&'static str, fn()>;
}

pub fn init_features() -> Result<(), Box<dyn std::error::Error>> {
    let registry: &Mutex<HashMap<&'static str, fn()>> =
        GLOBAL_REGISTRY.get_or_init(|| Mutex::new(HashMap::new()));

    let mut features: MutexGuard<'_, HashMap<&'static str, fn()>> = registry.lock()?;

    features.extend(
        &module01_mutability_rules::MutabilityRulesModuleFeatureRegister::default().get_features(),
    );

    features.extend(
        &module02_data_structures::DataStructureVecModuleFeatureRegister::default().get_features(),
    );

    features.extend(
        &module02_data_structures::DataStructureHashMapModuleFeatureRegister::default()
            .get_features(),
    );

    features.extend(
        &module02_data_structures::DataStructureHashSetModuleFeatureRegister::default()
            .get_features(),
    );

    features.extend(
        &module09_smart_pointers::SmartPointerBoxModuleFeatureRegister::default().get_features(),
    );

    Ok(())
}

pub fn get_all_features()
-> Result<MutexGuard<'static, HashMap<&'static str, fn()>>, Box<dyn std::error::Error>> {
    let registry: &Mutex<HashMap<&'static str, fn()>> =
        GLOBAL_REGISTRY.get_or_init(|| Mutex::new(HashMap::new()));

    Ok(registry.lock()?)
}
