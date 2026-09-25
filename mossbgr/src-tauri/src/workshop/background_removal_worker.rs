use image::imageops::FilterType;
use image::{DynamicImage, GenericImageView, RgbaImage};

use crate::contracts::SegmentationModelTrait;

pub struct BackgroundRemovalWorker<M: SegmentationModelTrait> {
    segmentation_model: M,
    current_image: Option<DynamicImage>,
    last_result: Option<RgbaImage>,
}

impl<M: SegmentationModelTrait> BackgroundRemovalWorker<M> {
    pub fn new(segmentation_model: M) -> Self {
        Self {
            segmentation_model,
            current_image: None,
            last_result: None,
        }
    }

    pub fn set_current_image(&mut self, image: DynamicImage) {
        self.current_image = Some(image);
    }

    pub fn current_image(&self) -> Option<&DynamicImage> {
        self.current_image.as_ref()
    }

    pub fn remove_background(&mut self) -> Result<RgbaImage, String> {
        let original = self
            .current_image
            .as_ref()
            .ok_or_else(|| "No image loaded — pick an image first".to_string())?;

        let mask = self.segmentation_model.predict_mask(original)?;
        let (width, height) = original.dimensions();
        let upscaled_mask = image::imageops::resize(&mask, width, height, FilterType::Triangle);

        let mut result = original.to_rgba8();
        for (pixel, mask_pixel) in result.pixels_mut().zip(upscaled_mask.pixels()) {
            pixel.0[3] = mask_pixel.0[0];
        }

        self.last_result = Some(result.clone());

        Ok(result)
    }

    pub fn last_result(&self) -> Option<&RgbaImage> {
        self.last_result.as_ref()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use image::{GrayImage, Luma};

    struct MockSegmentationModel;

    impl SegmentationModelTrait for MockSegmentationModel {
        fn predict_mask(&self, _image: &DynamicImage) -> Result<GrayImage, String> {
            let mut mask = GrayImage::new(2, 1);
            mask.put_pixel(0, 0, Luma([0]));
            mask.put_pixel(1, 0, Luma([255]));
            Ok(mask)
        }
    }

    #[test]
    fn remove_background_fails_when_no_image_loaded() {
        let mut worker = BackgroundRemovalWorker::new(MockSegmentationModel);

        let result = worker.remove_background();

        assert!(result.is_err());
    }

    #[test]
    fn remove_background_upscales_mask_and_composites_alpha() {
        let mut worker = BackgroundRemovalWorker::new(MockSegmentationModel);
        worker.set_current_image(DynamicImage::ImageRgba8(RgbaImage::from_pixel(
            4,
            2,
            image::Rgba([200, 100, 50, 255]),
        )));

        let result = worker.remove_background().unwrap();

        assert_eq!(result.dimensions(), (4, 2));
        assert_eq!(result.get_pixel(0, 0).0[3], 0);
        assert_eq!(result.get_pixel(3, 0).0[3], 255);
        assert_eq!(&result.get_pixel(0, 0).0[..3], &[200, 100, 50]);
    }

    #[test]
    fn last_result_reflects_the_most_recent_removal() {
        let mut worker = BackgroundRemovalWorker::new(MockSegmentationModel);
        assert!(worker.last_result().is_none());

        worker.set_current_image(DynamicImage::ImageRgba8(RgbaImage::new(2, 1)));
        worker.remove_background().unwrap();

        assert!(worker.last_result().is_some());
    }

    #[test]
    fn set_current_image_then_current_image_round_trips() {
        let mut worker = BackgroundRemovalWorker::new(MockSegmentationModel);
        assert!(worker.current_image().is_none());

        worker.set_current_image(DynamicImage::ImageRgba8(RgbaImage::new(10, 10)));

        assert!(worker.current_image().is_some());
    }
}
