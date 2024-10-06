#[tokio::test]
async fn test_crud_operations_in_order() {
    let client = reqwest::Client::new();

    // 1. Test POST: Añadir un equipo
    {
        let new_team = serde_json::json!({
            "name": "Example Team",
            "drivers": [
                {
                    "name": "Nombre Apellido",
                    "age": 23,
                    "nationality": "British"
                },
                {
                    "name": "Nombre Apellido2",
                    "age": 22,
                    "nationality": "Australian"
                }
            ]
        });

        let response = client
            .post("http://localhost:7000/api/escuderias")
            .json(&new_team)
            .send()
            .await
            .expect("Failed to send POST request");

        assert_eq!(response.status(), 201, "El equipo debería añadirse correctamente");
    }

    // 2. Test PUT: Actualizar el equipo añadido
    {
        let updated_team = serde_json::json!({
            "name": "Example Team",
            "drivers": [
                {
                    "name": "Max Verstappen5",
                    "age": 26,
                    "nationality": "Dutch5"
                },
                {
                    "name": "Sergio Perez5",
                    "age": 34,
                    "nationality": "Mexican5"
                }
                
            ]
        });

        let response = client
            .put("http://localhost:7000/api/escuderias/Example Team")
            .json(&updated_team)
            .send()
            .await
            .expect("Failed to send PUT request");

        assert_eq!(response.status(), 200, "El equipo debería actualizarse correctamente");
    }

    // 3. Test PATCH: Actualizar un conductor específico
    {
        let updated_driver = serde_json::json!({
            "age": 35,
            "nationality": "British"
        });

        let response = client
            .patch("http://localhost:7000/api/escuderias/Example Team/pilotos/Max Verstappen5")
            .json(&updated_driver)
            .send()
            .await
            .expect("Failed to send PATCH request");

        assert_eq!(response.status(), 200, "Los datos del conductor deberían actualizarse correctamente");
    }

    // 5. Test DELETE: Eliminar el equipo añadido
    {
        let response = client
            .delete("http://localhost:7000/api/escuderias/Example Team")
            .send()
            .await
            .expect("Failed to send DELETE request");

        assert_eq!(response.status(), 200, "El equipo debería eliminarse correctamente");
    }
}

#[tokio::test]
async fn test_get_team() {
    let client = reqwest::Client::new();



    // Probar el GET para obtener el equipo creado
    let get_response = client
        .get("http://localhost:7000/api/escuderias")
        .send()
        .await
        .expect("Failed to send GET request");

    assert_eq!(get_response.status(), 200, "Debería obtenerse el equipo correctamente");
}