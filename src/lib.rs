pub struct Tensor<T> {
    shape: Vec<usize>,
    content: Vec<T>,
    content_size: usize,
}

#[cfg(test)]
mod tests {
    use super::Tensor;

    #[test]
    fn tensor_initializer() {
        // Initialize tensor
        let shape: Vec<usize> = vec![2, 2];
        let content_size: usize = shape.iter().fold(0, |acc, &x| acc + x);
        let mut content: Vec<f32> = Vec::with_capacity(content_size);
        for i in 0..content_size {
            content.push(0.0);
        }

        let t: Tensor<f32> = Tensor{
            shape: shape,
            content_size: content_size,
            content: content,
        };

        assert_eq!(t.shape, vec![2, 2]);
        assert_eq!(t.content, vec![0.0, 0.0, 0.0, 0.0]);
    }
}
