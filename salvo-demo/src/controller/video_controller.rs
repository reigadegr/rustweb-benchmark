use crate::services::r#impl::video_service_impl::VideoServiceImpl;
use crate::services::video_service::VideoService;
use salvo::{handler, Request, Response};

#[handler]
pub async fn get_video_current_tine(req: &mut Request, res: &mut Response) {
    <VideoServiceImpl as VideoService>::get_video_current_tine(req, res).await
}
