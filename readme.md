# Rust Images Editor

## Commands

Write to File
````
cargo run -- write-to-file --filepath test-output/hello.txt --content "hello world"
````

Grid
```
cargo run -- grid --filepath test-images/dog.png --rows 3 --columns 4
```

Image Details
```
cargo run -- image-details --filepath test-images/dog2.png    
```

Gradient
```
cargo run -- gradient --savefilepath test-output/gradient_test.png    
```