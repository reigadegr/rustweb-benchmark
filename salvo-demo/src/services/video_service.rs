use salvo::{Request, Response};
pub trait VideoService {
    async fn get_video_current_tine(req: &mut Request, res: &mut Response);
}
