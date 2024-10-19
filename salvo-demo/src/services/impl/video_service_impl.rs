use crate::config::redis::{redis_read, redis_write_and_rm};
use crate::res::result::ResponseData;
use crate::services::video_service::VideoService;
use salvo::{Request, Response};

pub struct VideoServiceImpl;
impl VideoService for VideoServiceImpl {
    async fn get_video_current_tine(req: &mut Request, res: &mut Response) {
        let rs = redis_read("now_video_time").await;
        println!("{:?}", rs);
        if rs.is_ok() {
            let data: ResponseData<()> = ResponseData::success((), "已有，无需写入");
            return res.render(serde_json::to_string(&data).unwrap());
        }
        let time = req.param::<String>("time").unwrap();
        let rs = redis_write_and_rm("now_video_time", &time, 1).await;
        if rs.is_err() {
            let data: ResponseData<()> = ResponseData::error("无法连接redis");
            return res.render(serde_json::to_string(&data).unwrap());
        }
        let data = ResponseData::success(time, "写入成功");
        res.render(serde_json::to_string(&data).unwrap())
    }
}
