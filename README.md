# Rust Matrix
### Goal: 
The goal of this project is to create a library for **personal** use that adds the matrix data structure. The structure 
should implement traits allowing for the matrix to be indexed, summed, subracted, multiplied, and potentially divided.
Furthermore, the matrix structure should be generic to an extent allowing for different numerical types to be stored
within it.

### TODO:
- [ ] Write a tests to ensure that as the project is written it is working as expected
- [x] Create a matrix structure which can accept multiple numerical data types
    - [x] Implement display trait
    - [ ] Implement a trait allowing for the matrices to be indexed
    - [ ] Implement addition and subtraction to the matrices
    - [ ] Implement multiplication (and division) for the matrix structure
        - [ ] Method to multiply matrix by integer
        - [ ] Method to multiply matrix by matrix
- [ ] Optimise operations such as addition for the matrix
- [ ] Write a macro allowing for multiple matrices to be multiplied together
