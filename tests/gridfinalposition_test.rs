#[cfg(test)]
mod gridfinalposition_test {
    use rustimageseditor::services::grid_service::get_final_size;

    #[test]
    fn should_be_0() {
        let final_size : u32 = get_final_size(0, 0, 0);
        assert_eq!(final_size == 0, true)
    }

    // image is 100 pixels long, the cell size is 20 pixels long, the position is 90
    // Crop an image 20 pixels long from position 90 to 120. Image size is only 100 pixels long
    // we need to reduce the crop length to 10, otherwise it will fail
    #[test]
    fn should_be_10() {
        let final_size: u32 = get_final_size(90, 20, 100);
        assert_eq!(final_size == 10, true)
    }

    // image is 315 pixels long, the cell size is 40 pixels long, the position is 300
    // Crop an image 40 pixels long from position 300 to 340. Image size is only 315 pixels long
    // we need to reduce the crop length to 25, otherwise it will fail
    #[test]
    fn should_be_25() {
        let final_size: u32 = get_final_size(300, 40, 315);
        assert_eq!(final_size == 25, true)
    }

    #[test]
    fn should_be_return_same_size() {
        let final_size: u32 = get_final_size(50, 10, 100);
        assert_eq!(final_size == 10, true)
    }

    fn get_list_of_images_sizes(position: u32, image_size: u32, rows: i32) -> Vec<u32> {
        let mut lst: Vec<u32> = Vec::new();
        let mut current_position: u32 = position;
        let cell_size = image_size / rows as u32;

        for _ in 1..=rows {
            let temp = get_final_size(current_position, cell_size, image_size);
            lst.push(temp);
            current_position += cell_size;
        }

        let sum : u32 = lst.iter().sum();

        if sum < image_size {
            lst.push(image_size - sum);
        }

        lst
    }

    #[test]
    fn sum_of_sizes_should_pass_1() {
        let image_size : u32 = 100;
        let lst: Vec<u32> = get_list_of_images_sizes(0, image_size, 5);
        let sum : u32 = lst.iter().sum();

        println!("{:?}", lst);
        println!("{:?}", lst.len());
        println!("{:?}", sum);

        assert_eq!(sum, image_size);
    }

    #[test]
    fn sum_of_sizes_should_pass_2() {
        let image_size : u32 = 100;
        let lst: Vec<u32> = get_list_of_images_sizes(0,image_size, 6);
        let sum : u32 = lst.iter().sum();

        println!("{:?}", lst);
        println!("{:?}", lst.len());
        println!("{:?}", sum);

        assert_eq!(sum, image_size);
    }

    #[test]
    fn sum_of_sizes_should_pass_3() {
        let image_size : u32 = 100;
        let lst: Vec<u32> = get_list_of_images_sizes(0,image_size, 8);
        let sum : u32 = lst.iter().sum();

        println!("{:?}", lst);
        println!("{:?}", lst.len());
        println!("{:?}", sum);

        assert_eq!(sum, image_size);
    }

    #[test]
    fn sum_of_sizes_should_pass_4() {
        let image_size : u32 = 100;
        let lst: Vec<u32> = get_list_of_images_sizes(0,image_size, 2);
        let sum : u32 = lst.iter().sum();

        println!("{:?}", lst);
        println!("{:?}", lst.len());
        println!("{:?}", sum);

        assert_eq!(sum, image_size);
    }


}