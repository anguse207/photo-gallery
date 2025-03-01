// atomic bool: started - true if the state machine has started (start a clock for image sending)

use std::{
    collections::VecDeque,
    sync::{atomic::AtomicBool, Arc, Mutex},
};
use tokio::sync::broadcast;
use tracing::debug;

use crate::{runtime, upload::PATH_PREFIX};

type Image = Vec<u8>;
type ImagePath = String;

#[derive(Debug, Clone)]
pub struct AppState {
    started: Arc<AtomicBool>,
    images: Arc<Mutex<VecDeque<ImagePath>>>,
    pub tx: broadcast::Sender<Image>,
}

impl AppState {
    pub fn new() -> Self {
        let (tx, _) = broadcast::channel(32);

        Self {
            started: Arc::new(AtomicBool::new(false)),
            images: Arc::new(Mutex::new(VecDeque::new())),
            tx,
        }
    }

    pub async fn start(&self) {
        self.started
            .store(true, std::sync::atomic::Ordering::Relaxed);
        runtime::start(
            self.clone(),
            std::env::var("IMAGE_DURATION").unwrap().parse().unwrap(),
        )
        .await;
    }

    pub fn try_next_image(&self) {
        let mut images = self.images.lock().unwrap();

        if images.is_empty() {
            debug!("No images to send");
            return;
        }

        // Get the first image from the queue
        let image_name = images.pop_front().unwrap();
        let image = self.get_image_bytes(&image_name);

        // Remove the image from the filesystem
        let path = format!("{}{}", *PATH_PREFIX, &image_name);
        debug!("Removing image {:?}", &path);
        std::fs::remove_file(&path).unwrap();

        // Send the image to the frontend, via the broadcast channel
        match self.tx.send(image) {
            Ok(_) => debug!("Image sent to receivers"),
            Err(_) => debug!("No receivers"),
        }
    }

    pub fn is_started(&self) -> bool {
        self.started.load(std::sync::atomic::Ordering::Relaxed)
    }

    pub fn add_image(&self, image_name: &ImagePath) {
        self.images.lock().unwrap().push_back(image_name.clone());
    }

    fn get_image_bytes(&self, image_name: &ImagePath) -> Image {
        std::fs::read(format!("{}{}", *PATH_PREFIX, &image_name)).unwrap()
    }
}
