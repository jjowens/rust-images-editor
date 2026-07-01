#[cfg(test)]
mod gradient_test {
    use assert_cmd::Command;
    use chrono::Local;

    const APP_NAME: &str = "rustimageseditor";

    #[test]
    fn get_gradient_1() {
        let mut cmd = Command::cargo_bin(APP_NAME).unwrap();

        cmd.arg("gradient")
            .arg("--savefilepath").arg("test-output/gradent_1.png")
            .arg("--width").arg("100")
            .arg("--height").arg("100")
            .arg("--transparency").arg("4");

        let output = cmd.unwrap();
    }

    fn run_gradient_rgba(save_file_path: &str,
                         width: u32,
                         height: u32,
                         red: u32,
                         green: u32,
                         blue: u32,
                         alpha: u8,
                         redy: bool,
                         greeny: bool,
                         bluey: bool) {
        let mut cmd = Command::cargo_bin(APP_NAME).unwrap();

        cmd.arg("gradient-rgba")
            .arg("--savefilepath").arg(save_file_path)
            .arg("--width").arg(width.to_string())
            .arg("--height").arg(height.to_string())
            .arg("--red").arg(red.to_string())
            .arg("--green").arg(green.to_string())
            .arg("--blue").arg(blue.to_string())
            .arg("--alpha").arg(alpha.to_string())
            .arg("--redy").arg(redy.to_string())
            .arg("--greeny").arg(greeny.to_string())
            .arg("--bluey").arg(bluey.to_string());

        let output = cmd.unwrap();
    }

    #[test]
    fn get_gradient_rgba_red() {
        let output = run_gradient_rgba("test-output/gradentrgba_red.png", 100, 100, 255, 0, 0, 100, false, false, true);
    }

    #[test]
    fn get_gradient_rgba_green() {
        let output = run_gradient_rgba("test-output/gradentrgba_green.png", 100, 100, 0, 255, 0, 100, false, false, true);
    }

    #[test]
    fn get_gradient_rgba_blue() {
        let output = run_gradient_rgba("test-output/gradentrgba_blue.png", 100, 100, 0, 0, 255, 100, false, false, true);
    }

    #[test]
    fn get_gradient_rgba_redx200() {
        let output = run_gradient_rgba("test-output/gradentrgba_redx200.png", 100, 100, 255, 0, 0, 200, false, false, true);
    }

    #[test]
    fn get_gradient_rgba_redx255() {
        let output = run_gradient_rgba("test-output/gradentrgba_redx255.png", 100, 100, 255, 0, 0, 255, false, false, true);
    }

    #[test]
    fn get_gradient_rgba_redgreenx255() {
        let output = run_gradient_rgba("test-output/gradentrgba_redgreenx255.png", 100, 100, 255, 255, 0, 255, false, false, true);
    }

    #[test]
    fn get_gradient_rgba_redyx255() {
        let outcome = run_gradient_rgba("test-output/gradentrgba_redyx200.png",
                                  100,
                                  100,
                                  255,
                                  0,
                                  0,
                                  255,
                                  true,
                                  false,
                                  false);
    }
    #[test]
    fn get_gradient_rgba_redygreenyx255() {
        let output = run_gradient_rgba("test-output/gradentrgba_redygreenyx200.png",
                                  100,
                                  100,
                                  255,
                                  255,
                                  0,
                                  255,
                                  true,
                                  true,
                                  false);
    }

    #[test]
    fn get_gradient_rgba_redygreenx255() {
        let output = run_gradient_rgba("test-output/gradentrgba_redygreenx200.png",
                                  100,
                                  100,
                                  255,
                                  255,
                                  0,
                                  255,
                                  true,
                                  false,
                                  false);
    }

    #[test]
    fn get_gradient_rgba_red_vector()  {
        let vecs: Vec<u32> = vec![10,20,30,40,50,60,70,80,90,100, 110, 120, 130, 140, 150, 160, 170, 180, 190, 200, 210, 220, 230, 240, 250, 255];

        for vec in vecs {
            let save_file_path = format!("test-output/gradientrgba_red_{}.png", vec);
            let output = run_gradient_rgba(save_file_path.as_str(),
                                      100,
                                      100,
                                      vec,
                                      0,
                                      0,
                                      255,
                                      false,
                                      false,
                                      true);
        }
    }

    #[test]
    fn get_gradient_rgba_green_vector() {
        let vecs: Vec<u32> = vec![10,20,30,40,50,60,70,80,90,100, 110, 120, 130, 140, 150, 160, 170, 180, 190, 200, 210, 220, 230, 240, 250, 255];

        for vec in vecs {
            let save_file_path = format!("test-output/gradientrgba_green_{}.png", vec);
            let output = run_gradient_rgba(save_file_path.as_str(),
                                      100,
                                      100,
                                      0,
                                      vec,
                                      0,
                                      255,
                                      false,
                                      false,
                                      true);
        }
    }

    #[test]
    fn get_gradient_rgba_blue_vector() {
        let vecs: Vec<u32> = vec![10,20,30,40,50,60,70,80,90,100, 110, 120, 130, 140, 150, 160, 170, 180, 190, 200, 210, 220, 230, 240, 250, 255];

        for vec in vecs {
            let save_file_path = format!("test-output/gradientrgba_blue_{}.png", vec);
            let output = run_gradient_rgba(save_file_path.as_str(),
                                      100,
                                      100,
                                      0,
                                      0,
                                      vec,
                                     255,
                                      true,
                                      false,
                                      false);
        }
    }

    #[test]
    fn get_gradient_rgba_bluey_vector() {
        let vecs: Vec<u32> = vec![10,20,30,40,50,60,70,80,90,100, 110, 120, 130, 140, 150, 160, 170, 180, 190, 200, 210, 220, 230, 240, 250, 255];

        for vec in vecs {
            let save_file_path = format!("test-output/gradientrgba_bluey_{}.png", vec);
            let output = run_gradient_rgba(save_file_path.as_str(),
                                      100,
                                      100,
                                      0,
                                      0,
                                      vec,
                                      255,
                                      false,
                                      false,
                                      true);
        }
    }

    #[test]
    fn get_gradient_random() {
        let mut cmd = Command::cargo_bin(APP_NAME).unwrap();
        let dt = Local::now();

        let save_file_path = format!("test-output/gradient-random/gradient_random_{}.png", dt.format("%Y%m%d%H%M"));
        cmd.arg("gradient-random")
            .arg("--savefilepath").arg(save_file_path)
            .arg("--width").arg("200")
            .arg("--height").arg("200");

        let output = cmd.unwrap();

    }

    #[test]
    fn get_gradient_random_block() {
        let mut cmd = Command::cargo_bin(APP_NAME).unwrap();
        let dt = Local::now();

        let save_file_path = format!("test-output/gradient-random/gradient_block_{}.png", dt.format("%Y%m%d%H%M"));
        cmd.arg("gradient-block")
            .arg("--savefilepath").arg(save_file_path)
            .arg("--width").arg("200")
            .arg("--height").arg("200");

        let output = cmd.unwrap();
    }

    #[test]
    fn get_gradient_custom() {
        let mut cmd = Command::cargo_bin(APP_NAME).unwrap();
        let dt = Local::now();

        let save_file_path = format!("test-output/gradient-random/gradient_block_{}.png", dt.format("%Y%m%d%H%M"));
        cmd.arg("gradient-block")
            .arg("--savefilepath").arg(save_file_path)
            .arg("--width").arg("200")
            .arg("--height").arg("200");

        let output = cmd.unwrap();
    }

}