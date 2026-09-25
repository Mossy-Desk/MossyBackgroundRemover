use std::path::PathBuf;

use crate::contracts::FileDialogTrait;

pub struct DialogWorker<D: FileDialogTrait> {
    file_dialog: D,
}

impl<D: FileDialogTrait> DialogWorker<D> {
    pub fn new(file_dialog: D) -> Self {
        Self { file_dialog }
    }

    /// `Ok(None)` means the user cancelled — not an error.
    pub fn pick_image_file(&self) -> Result<Option<PathBuf>, String> {
        self.file_dialog.pick_image_file()
    }

    /// `Ok(None)` means the user cancelled — not an error.
    pub fn pick_save_path(&self, suggested_file_name: &str) -> Result<Option<PathBuf>, String> {
        self.file_dialog.pick_png_save_path(suggested_file_name)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;

    struct MockDialog {
        next_pick: RefCell<Option<PathBuf>>,
        next_save: RefCell<Option<PathBuf>>,
    }

    impl FileDialogTrait for MockDialog {
        fn pick_image_file(&self) -> Result<Option<PathBuf>, String> {
            Ok(self.next_pick.borrow_mut().take())
        }

        fn pick_png_save_path(
            &self,
            _suggested_file_name: &str,
        ) -> Result<Option<PathBuf>, String> {
            Ok(self.next_save.borrow_mut().take())
        }
    }

    #[test]
    fn pick_image_file_returns_none_when_dialog_cancelled() {
        let worker = DialogWorker::new(MockDialog {
            next_pick: RefCell::new(None),
            next_save: RefCell::new(None),
        });

        assert!(worker.pick_image_file().unwrap().is_none());
    }

    #[test]
    fn pick_image_file_returns_the_chosen_path() {
        let worker = DialogWorker::new(MockDialog {
            next_pick: RefCell::new(Some(PathBuf::from("photo.png"))),
            next_save: RefCell::new(None),
        });

        assert_eq!(
            worker.pick_image_file().unwrap(),
            Some(PathBuf::from("photo.png"))
        );
    }

    #[test]
    fn pick_save_path_returns_none_when_dialog_cancelled() {
        let worker = DialogWorker::new(MockDialog {
            next_pick: RefCell::new(None),
            next_save: RefCell::new(None),
        });

        assert!(worker.pick_save_path("result.png").unwrap().is_none());
    }

    #[test]
    fn pick_save_path_returns_the_chosen_path() {
        let worker = DialogWorker::new(MockDialog {
            next_pick: RefCell::new(None),
            next_save: RefCell::new(Some(PathBuf::from("/tmp/result.png"))),
        });

        assert_eq!(
            worker.pick_save_path("result.png").unwrap(),
            Some(PathBuf::from("/tmp/result.png"))
        );
    }
}
