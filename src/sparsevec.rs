use std::collections::HashMap; // for dictionaries

/// ### cosine_similarity
/// This method takes two sparse vectors represented as <u32, f32> HashMaps
/// It returns their similary as defined by (dot_product)/(norm_a * norm_b)
pub fn cosine_similarity(u: HashMap<u32, f32>, v: HashMap<u32, f32>) -> f32 {
    // return the similarity of two sparse vectors as defined by (u*v)/(||u||*||v||)

    let mut dot_prod: f32 = 0f32;      // dot product
    let mut u_norm: f32 = 0f32;    // norm of vector u
    let mut v_norm: f32 = 0f32;     // norm of vector v

    for (key, u_element) in &u {
        let v_element = match &v.get(&key){
            Some(element) => element,
            None => &0f32,
        };
        dot_prod = dot_prod + (u_element * v_element);
        u_norm = u_norm + u_element;
    }
    for (_, v_element) in &v{
        v_norm = v_norm + v_element;
    }

    // calculate and return the similarity
    let similarity:f32 = dot_prod/(u_norm * v_norm); // as percentage
    similarity

}
