use salvo::{Request, Response};

// 定义 UserService trait 作为接口
pub trait UsersService {
    async fn login_post(req: &mut Request, res: &mut Response);
    async fn users_info(res: &mut Response);
    async fn users_logout(res: &mut Response);
    async fn users_get_all(req: &mut Request, res: &mut Response);
}
