use ndarray::Array2;

pub(super) fn vec_to_array<T: Clone>(vec: &[Vec<T>]) -> Array2<T> {
    debug!("Converting vec<vec<T> to ndarray.");
    if vec.is_empty() {
        return Array2::from_shape_vec((0, 0), Vec::new()).unwrap();
    }
    let total_rows = vec.len();
    let total_columns = vec[0].len();
    let mut data = Vec::with_capacity(total_rows * total_columns);
    for row in vec {
        debug!("Current row length is {}", row.len());
        data.extend_from_slice(row);
    }
    debug!(
        "Going to create array with shape, rows: {}, cols: {}",
        total_rows, total_columns
    );
    Array2::from_shape_vec((total_rows, total_columns), data).unwrap()
}

pub(super) fn pad_all_rows<T: Copy>(vec: &mut Vec<Vec<T>>, max_x_len: usize, padded_value: T) {
    debug!("Padding up to max x len: {}", max_x_len);
    for row in vec {
        while row.len() < max_x_len {
            row.push(padded_value);
        }
    }
}
