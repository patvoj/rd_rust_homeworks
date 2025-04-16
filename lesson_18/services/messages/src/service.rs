use crate::repository::MessageRepository;
use sqlx::PgPool;

pub struct MessageService {
    pub repo: MessageRepository,
    pub db: PgPool,
}

impl MessageService {
    pub fn new(db: PgPool) -> MessageService {
        let repo = MessageRepository::new(db.clone());

        return MessageService { repo, db };
    }
}
