use socket::{Request, Responder};

use crate::{
    error::AppError,
    protocol::command::{Message, Ping},
};

pub async fn ping(req: Request<Ping>, res: Responder) -> Result<(), AppError> {
    let _ = res.send(Message::Ping(req.body)).await;
    Ok(())
}
