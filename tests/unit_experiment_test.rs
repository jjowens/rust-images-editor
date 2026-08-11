#[cfg(test)]
mod unit_experiment_test {
    use rustimageseditor::services::experiment_service::{experiment_subimage_service, experiment_warhol_service};

    #[test]
    fn experiment_subimage_test() {
        let open_file_path = "test-images/experiment/dog1.png".to_string();
        let save_file_path = "test-output/experiment/dog_subimage.png".to_string();

        let output = experiment_subimage_service(open_file_path, save_file_path);

        assert!(output.is_ok());
    }

    #[test]
    fn experiment_warhol_test() {
        let open_file_path = "test-images/experiment/dog1.png".to_string();
        let save_file_path = "test-output/experiment/dog_warhol.png".to_string();

        let output = experiment_warhol_service(open_file_path, save_file_path, "dog_warhol".to_string());

        assert!(output.is_ok());
    }
}