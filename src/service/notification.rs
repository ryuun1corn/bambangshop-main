use std::thread;

use crate::model::notification::Notification;
use crate::model::product::Product;
use crate::model::subscriber::Subcriber;
use crate::repository::subscriber::SusbcriberRepository;
use bambangshop::{compose_error_response, Result};
use rocket::http::Status;

pub struct NotificationService;

impl NotificationService {}
