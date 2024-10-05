// tests/tests.rs
use serde_json::{json, Value};
use std::collections::HashMap;
use std::error::Error;

use http_server_proyecto1_so::http_functions;

#[test]
fn test_post_team() {
    let mut f1_data: HashMap<String, Value> = HashMap::new();
    f1_data.insert("teams".to_string(), json!([])); // Inicializar equipos vacíos

    let new_team = json!({"name": "Team A"});

    // Probar agregar un nuevo equipo
    let result = http_functions::post_team(new_team.clone(), f1_data.clone());
    assert!(result.is_ok());

    // Probar agregar el mismo equipo nuevamente (debería fallar)
    let result = http_functions::post_team(new_team, f1_data);
    assert!(result.is_err());
}

#[test]
fn test_put_team() {
    let mut f1_data: HashMap<String, Value> = HashMap::new();
    f1_data.insert("teams".to_string(), json!([{"name": "Team A"}]));

    let updated_team = json!({"name": "Team A", "score": 10});

    // Probar actualizar un equipo existente
    let result = http_functions::put_team(updated_team.clone(), f1_data.clone());
    assert!(result.is_ok());

    // Probar agregar un equipo nuevo
    let new_team = json!({"name": "Team B"});
    let result = http_functions::put_team(new_team, f1_data);
    assert!(result.is_err());
}

#[test]
fn test_delete_team() {
    let mut f1_data: HashMap<String, Value> = HashMap::new();
    f1_data.insert("teams".to_string(), json!([{"name": "Team A"}]));

    // Probar eliminar un equipo existente
    let result = http_functions::delete_team("Team A", f1_data.clone());
    assert!(result.is_ok());

    // Probar eliminar un equipo que no existe
    let result = http_functions::delete_team("Team B", f1_data);
    assert!(result.is_err());
}

#[test]
fn test_patch_driver() {
    let mut f1_data: HashMap<String, Value> = HashMap::new();
    f1_data.insert("teams".to_string(), json!([{
        "name": "Team A",
        "drivers": [{"name": "Driver 1", "points": 0}]
    }]));

    let updated_data = json!({"points": 10});

    // Probar actualizar un conductor existente
    let result = http_functions::patch_driver("Team A", "Driver 1", updated_data.clone(), f1_data.clone());
    assert!(result.is_ok());

    // Probar actualizar un conductor que no existe
    let result = http_functions::patch_driver("Team A", "Driver 2", updated_data, f1_data);
    assert!(result.is_err());

    // Probar actualizar un conductor en un equipo que no existe
    let result = http_functions::patch_driver("Team B", "Driver 1", updated_data, f1_data);
    assert!(result.is_err());
}