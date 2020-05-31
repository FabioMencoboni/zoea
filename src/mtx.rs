
/// ### DMatrix operations
/// The nalgebra crate contains greate functionality for DMatrix manipulations.
/// The mtx module of **Zoea** makes it easy to create nalgebra matrices.
/// 
/// ### Quickstart:
/// Create a new 5x5 f64 identity matrix and print the first column
/// ```
/// use zoea::mtx;
/// let mut m = mtx::new_f64_identity(5);
/// println!("Here is the first column: {}", m.column(0));
/// ```
/// 
/// Create a 3x1000 f32 random matrix with values between -1 and 1
/// Create a 1000x4 f32 random matrix with values between 5 and 25
/// 
/// ```
/// use zoea::mtx;
/// let a: mtx::DMatrix<f32> = mtx::new_f32_random(3, 1000, -1f32, 1f32);
/// let b: mtx::DMatrix<f32> = mtx::new_f32_random(1000, 4, 5f32, 25f32);
/// // multiply a and b and print the result
/// let c: mtx::DMatrix<f32> = a * b;
/// println!("{}", c);
/// 
/// // take one of the values and assign it to a float
/// let select_element: f32 = c[(1,3)];
/// ```


pub use nalgebra::DMatrix;
use rand::Rng;

/// ### new_f32_from_vec
/// Create a new DMatrix of f32 from a vector of f32
/// 
/// #### EXAMPLE:
/// ```
/// use zoea::mtx;
/// let mut m: mtx::DMatrix<f32> = mtx::new_f32_from_vec(2, 3, vec![1f32, 2f32, 3f32, 4f32, 5f32, 6f32]);
/// ```
pub fn new_f32_from_vec(rows: usize, cols: usize, vec: Vec<f32>) -> DMatrix<f32> {
    // create a DMatrix from a vector of f32
    let m: DMatrix<f32> = DMatrix::from_row_slice(rows,cols, &vec);
    m
}

/// ### new_f64_from_vec
/// Create a new DMatrix of f64 from a vector of f64
/// 
/// #### EXAMPLE:
/// ```
/// use zoea::mtx;
/// let mut m: mtx::DMatrix<f64> = mtx::new_f64_from_vec(2, 3, vec![1f64, 2f64, 3f64, 4f64, 5f64, 6f64]);
/// ```
pub fn new_f64_from_vec(rows: usize, cols: usize, vec: Vec<f64>) -> DMatrix<f64> {
    // create a DMatrix from a vector of f64
    let m: DMatrix<f64> = DMatrix::from_row_slice(rows,cols, &vec);
    m
}



/// ### new_f64_zeros
/// Create a new DMatrix of f64 zeros
/// The f32 equivalent is new_f32_zeros
/// 
/// #### EXAMPLE:
/// ```
/// use zoea::mtx;
/// let mut m: mtx::DMatrix<f64> = mtx::new_f64_zeros(2, 3);
/// ```
pub fn new_f64_zeros(rows: usize, cols: usize) -> DMatrix<f64> {
    let ct_elements: usize = rows * cols;
    let vec = vec![0f64; ct_elements];
    let m: DMatrix<f64> = DMatrix::from_row_slice(rows, cols, &vec);
    m
}
pub fn new_f32_zeros(rows: usize, cols: usize) -> DMatrix<f32> {
    let ct_elements: usize = rows * cols;
    let vec = vec![0f32; ct_elements];
    let m: DMatrix<f32> = DMatrix::from_row_slice(rows, cols, &vec);
    m
}


/// ### new_f64_identity
/// Create a new f64 identity DMatrix of dimensions size x size
/// The f32 equivalent is new_f32_identity
/// 
/// #### EXAMPLE:
/// ```
/// use zoea::mtx;
/// let mut m: mtx::DMatrix<f64> = mtx::new_f64_identity(5);
/// ```
pub fn new_f64_identity(size: usize) -> DMatrix<f64> {
    let m: DMatrix<f64> = DMatrix::identity(size, size);
    m
}
pub fn new_f32_identity(size: usize) -> DMatrix<f32> {
    let m: DMatrix<f32> = DMatrix::identity(size, size);
    m
}

/// ### new_f64_random
/// Create a new f64 random matrix of size (rows, cols) with values between (min, max)
/// The f32 equivalent is new_f32_random
/// 
/// #### EXAMPLE:
/// ```
/// use zoea::mtx;
/// let mut m: mtx::DMatrix<f64> = mtx::new_f64_random(2, 3, 0f64, 100f64);
/// ```

pub fn new_f64_random(rows: usize, cols: usize, min: f64, max: f64) -> DMatrix<f64> {
    let ct_elements: usize = rows * cols;
    let mut vec = vec![0f64; ct_elements];
    let mut rng = rand::thread_rng();
    for i in 0..vec.len() {
        vec[i] = min+ (max-min)*rng.gen::<f64>();
    }
    let m: DMatrix<f64> = DMatrix::from_row_slice(rows, cols, &vec);
    m
}
pub fn new_f32_random(rows: usize, cols: usize, min: f32, max: f32) -> DMatrix<f32> {
    let ct_elements: usize = rows * cols;
    let mut vec = vec![0f32; ct_elements];
    let mut rng = rand::thread_rng();
    for i in 0..vec.len() {
        vec[i] = min+ (max-min)*rng.gen::<f32>();
    }
    let m: DMatrix<f32> = DMatrix::from_row_slice(rows, cols, &vec);
    m
}

