# Rust Images Editor

## Commands

Write to File
````
cargo run -- write-to-file --filepath test-output/hello.txt --content "hello world"
````

Grid
```
cargo run -- grid --openfilepath test-images/dog1.png --rows 2 --columns 2 --savefilepath test-output/dog_grid.png
```

````
cargo run -- grid --openfilepath test-images/dog1.png --rows 2 --columns 2 --savefilepath test-output/dog_grid.png
````

Image Details
```
cargo run -- image-details --filepath test-images/dog2.png    
```

Gradient
```
cargo run -- gradient --savefilepath test-output/gradient_test.png    
```