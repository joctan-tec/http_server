extern crate http_server_proyecto1_so;

use http_server_proyecto1_so::write_to_temp_file;

    #[test]
    fn test_write_to_temp_file() {
        let result = write_to_temp_file("temp.txt", "contenido".to_string());
        
        assert!(result.is_ok());
    }