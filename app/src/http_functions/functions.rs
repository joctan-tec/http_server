use serde_json::Value;
use std::{collections::HashMap, error::Error};
use crate::json_hashmaps::f1_data_hashmap::write_json_to_file;

pub fn post_team(
    new_team: Value,
    f1_data: &mut HashMap<String, Value>, // Cambiar para tomar una referencia mutable
) -> Result<(), Box<dyn Error>> {
    let teams = f1_data.get_mut("teams").unwrap().as_array_mut().unwrap();

    // Verificar si el equipo ya existe
    if teams.iter().any(|team| team["name"] == new_team["name"]) {
        return Err("El equipo ya existe".into());
    }

    teams.push(new_team);
    write_json_to_file(f1_data);
    Ok(())
}

pub fn put_team(
    team_name: &str,
    new_team: Value,
    f1_data: &mut HashMap<String, Value>, // Cambiar para tomar una referencia mutable
) -> Result<(), Box<dyn Error>> {
    let teams: &mut Vec<Value> = f1_data.get_mut("teams").unwrap().as_array_mut().unwrap();

    // Verificar si el equipo existe
    if let Some(pos) = teams.iter().position(|team| team["name"] == team_name) {
        teams[pos] = new_team; // Actualizar el equipo
        write_json_to_file(f1_data);
        Ok(())
    } else {
        return Err(format!("El equipo '{}' no existe", team_name).into());
    }
}

pub fn delete_team(
    team_name: &str,
    f1_data: &mut HashMap<String, Value>, // Cambiar para tomar una referencia mutable
) -> Result<(), Box<dyn Error>> {
    let teams = f1_data.get_mut("teams").unwrap().as_array_mut().unwrap();

    // Verificar si el equipo existe
    if let Some(pos) = teams.iter().position(|team| team["name"] == team_name) {
        teams.remove(pos); // Eliminar el equipo si se encuentra
        write_json_to_file(f1_data);
        Ok(())
    } else {
        Err(format!("El equipo '{}' no existe", team_name).into()) // Retornar un error si no se encuentra
    }
}

pub fn patch_driver(
    team_name: &str,
    driver_name: &str,
    updated_data: Value,
    f1_data: &mut HashMap<String, Value>, // Cambiar para tomar una referencia mutable
) -> Result<(), Box<dyn Error>> {
    let teams = f1_data.get_mut("teams").unwrap().as_array_mut().unwrap();

    // Verificar si el equipo existe
    if let Some(team) = teams.iter_mut().find(|team| team["name"] == team_name) {
        let drivers = team.get_mut("drivers").unwrap().as_array_mut().unwrap();

        // Verificar si el conductor existe
        if let Some(driver) = drivers.iter_mut().find(|driver| driver["name"] == driver_name) {
            for (key, value) in updated_data.as_object().unwrap() {
                driver[key] = value.clone();
            }
            write_json_to_file(f1_data);
            Ok(())
        } else {
            Err(format!(
                "El conductor '{}' no existe en el equipo '{}'",
                driver_name, team_name
            ).into())
        }
    } else {
        Err(format!("El equipo '{}' no existe", team_name).into())
    }
}