pub fn img_checksum(pix_matrix: &Vec<Vec<u32>>) -> usize {
    let layer_fewest_zeros = pix_matrix
        .iter()
        .min_by_key(|layer| count_color(layer, 0))
        .unwrap();

    count_color(layer_fewest_zeros, 1) * count_color(layer_fewest_zeros, 2)
}

fn count_color(layer: &Vec<u32>, color: u32) -> usize {
    layer.iter().filter(|&&c| c == color).count()
}

pub fn combine_layers(pix_matrix: &Vec<Vec<u32>>, layer_len: usize) -> Vec<u32> {
    let mut combined = Vec::new();

    for i in 0..layer_len {
        let mut color = 2;
        let mut layer_it = pix_matrix.iter();

        // While we get only transparent
        while color == 2 {
            // Get the color on the next layer
            if let Some(layer) = layer_it.next() {
                color = layer[i];
            } else {
                break;
            }
        }

        combined.push(color);
    }

    combined
}
