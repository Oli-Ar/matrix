# Rust Matrix
### Goal: 
The goal of this project is to create a library for **personal** use that adds the matrix data structure to rust. 
The structure should implement traits allowing for the matrix to be indexed, summed, subracted, multiplied, and 
divided. Furthermore, the matrix structure should be generic to an extent of allowing different numerical types 
to be stored within it.

### TODO:
- [ ] Write tests to ensure that as the project is written it is working as expected
    - [x] Tests for matrix creation
    - [x] Indexing tests
    - [ ] Tests for addition and subtraction
    - [ ] Multipliaction operation test
- [x] Create a matrix structure which can accept multiple numerical data types
    - [x] Implement display trait
    - [x] Implement a trait allowing for the matrices to be indexed
        - [ ] Implement a method for indexing without the possibility of panicking
    - [ ] Implement addition and subtraction to the matrices
        - [x] Implement addition
        - [ ] Implement subtraction
    - [x] Implement multiplication for the matrix structure
        - [x] Implemet scalar multiplication for matrix
        - [x] Method to multiply matrix by a matrix
- [ ] Optimise operations such as addition for the matrix
    - [ ] Possibily add GPU acceleration
- [ ] Write a macro allowing for multiple matrices to be multiplied together
- [ ] Maybe write documentation
