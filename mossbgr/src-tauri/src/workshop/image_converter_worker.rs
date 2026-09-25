use std::path::Path;

use image::imageops::FilterType;
use image::{DynamicImage, GenericImageView};

use crate::contracts::{ImageCodecTrait, ImagePreviewDto};

const PREVIEW_MAX_DIMENSION: u32 = 1600;

pub struct ImageConverterWorker<C: ImageCodecTrait> {
    image_codec: C,
}

impl<C: ImageCodecTrait> ImageConverterWorker<C> {
    pub fn new(image_codec: C) -> Self {
        Self { image_codec }
    }

    pub fn decode_from_path(&self, path: &Path) -> Result<DynamicImage, String> {
        self.image_codec.load_from_path(path)
    }

    pub fn to_preview_dto(&self, image: &DynamicImage) -> Result<ImagePreviewDto, String> {
        let preview_image = downscale_for_preview(image);
        let png_base64 = self.image_codec.encode_png_base64(&preview_image)?;
        Ok(ImagePreviewDto {
            width: preview_image.width(),
            height: preview_image.height(),
            png_base64,
        })
    }

    pub fn encode_to_path(&self, image: &DynamicImage, path: &Path) -> Result<(), String> {
        self.image_codec.save_png_to_path(image, path)
    }
}

fn downscale_for_preview(image: &DynamicImage) -> DynamicImage {
    let (width, height) = image.dimensions();
    if width.max(height) <= PREVIEW_MAX_DIMENSION {
        return image.clone();
    }
    image.resize(
        PREVIEW_MAX_DIMENSION,
        PREVIEW_MAX_DIMENSION,
        FilterType::Triangle,
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use image::RgbaImage;
    use std::cell::RefCell;
    use std::path::PathBuf;

    struct MockCodec {
        decoded: DynamicImage,
        saved: RefCell<Vec<PathBuf>>,
    }

    impl MockCodec {
        fn new() -> Self {
            Self {
                decoded: DynamicImage::ImageRgba8(RgbaImage::new(10, 10)),
                saved: RefCell::new(Vec::new()),
            }
        }
    }

    impl ImageCodecTrait for MockCodec {
        fn load_from_path(&self, _path: &Path) -> Result<DynamicImage, String> {
            Ok(self.decoded.clone())
        }

        fn encode_png_base64(&self, image: &DynamicImage) -> Result<String, String> {
            Ok(format!("{}x{}", image.width(), image.height()))
        }

        fn save_png_to_path(&self, _image: &DynamicImage, path: &Path) -> Result<(), String> {
            self.saved.borrow_mut().push(path.to_path_buf());
            Ok(())
        }
    }

    #[test]
    fn decode_from_path_delegates_to_the_codec() {
        let worker = ImageConverterWorker::new(MockCodec::new());

        let image = worker.decode_from_path(Path::new("photo.png")).unwrap();

        assert_eq!((image.width(), image.height()), (10, 10));
    }

    #[test]
    fn to_preview_dto_leaves_small_images_untouched() {
        let worker = ImageConverterWorker::new(MockCodec::new());
        let image = DynamicImage::ImageRgba8(RgbaImage::new(100, 50));

        let dto = worker.to_preview_dto(&image).unwrap();

        assert_eq!((dto.width, dto.height), (100, 50));
    }

    #[test]
    fn to_preview_dto_downscales_large_images() {
        let worker = ImageConverterWorker::new(MockCodec::new());
        let image = DynamicImage::ImageRgba8(RgbaImage::new(4000, 2000));

        let dto = worker.to_preview_dto(&image).unwrap();

        assert!(dto.width <= PREVIEW_MAX_DIMENSION);
        assert!(dto.height <= PREVIEW_MAX_DIMENSION);
        // Aspect ratio preserved (2:1).
        assert_eq!(dto.width, dto.height * 2);
    }

    #[test]
    fn encode_to_path_delegates_to_the_codec() {
        let codec = MockCodec::new();
        let worker = ImageConverterWorker::new(codec);
        let image = DynamicImage::ImageRgba8(RgbaImage::new(2, 2));

        worker
            .encode_to_path(&image, Path::new("/tmp/out.png"))
            .unwrap();

        assert_eq!(
            worker.image_codec.saved.borrow().as_slice(),
            [PathBuf::from("/tmp/out.png")]
        );
    }
}
