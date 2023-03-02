use std::io::Cursor;
use tract_flavour::prelude::*;

// The inference function returns a tuple:
// (confidence, index of the predicted class)
pub fn infer(model_bytes: &[u8], image_bytes: &[u8]) -> TractResult<(f32, i32)> {
    println!("Optimizing Tensorflow model for F32 datum type, tensor shape [1, 224, 224, 3].");
    let model = tract_flavour::tensorflow() // swap in ::nnef() for the tract-nnef package, etc.
        // Load the model.
        .model_for_read(&mut Cursor::new(model_bytes))?
        // Specify input type and shape.
        .with_input_fact(
            0,
            InferenceFact::dt_shape(f32::datum_type(), tvec!(1, 224, 224, 3)),
        )?
        // Optimize the model.
        .into_optimized()?
        // Make the model runnable and fix its inputs and outputs.
        .into_runnable()?;

    // Create a new image from the image byte slice.
    let img = image::load_from_memory(image_bytes)?.to_rgb8();
    println!("Resizing image to fit 224x224 (filter algorithm: nearest neighbour).",);
    // Resize the input image to the dimension the model was trained on.
    // Sampling filter and performance comparison: https://docs.rs/image/0.23.12/image/imageops/enum.FilterType.html#examples
    // Switch to FilterType::Triangle if you're getting odd results.
    let resized = image::imageops::resize(&img, 224, 224, image::imageops::FilterType::Nearest);

    println!("Converting scaled image to tensor and running model...",);
    // Make a Tensor out of it.
    let img: Tensor = tract_ndarray::Array4::from_shape_fn((1, 224, 224, 3), |(_, y, x, c)| {
        resized[(x as _, y as _)][c] as f32 / 255.0
    })
    .into();

    // Run the model on the input.
    let result = model.run(tvec!(img))?;
    println!("Inference complete. Traversing results graph to find a best-confidence fit...");

    // Find the max value with its index.
    let best = result[0]
        .to_array_view::<f32>()?
        .iter()
        .cloned()
        .zip(1..)
        .max_by(|a, b| a.0.partial_cmp(&b.0).unwrap());

    Ok(best.unwrap())
}
