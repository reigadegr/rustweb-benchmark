use crate::controller::users_controller::{
    hello, login_post, users_get_all, users_info, users_logout,
};
use crate::controller::video_controller::get_video_current_tine;
use salvo::Router;

pub fn init_router() -> Router {
    Router::new()
        .push(Router::with_path("/").get(hello))
        .push(
            Router::new()
                .path("/users")
                .push(Router::new().path("info").get(users_info))
                .push(Router::new().path("login").post(login_post))
                .push(Router::new().path("logout").delete(users_logout))
                .push(Router::with_path("getall/<**>").get(users_get_all)),
        )
        .push(
            Router::new().path("/video").push(
                Router::new()
                    .path("/sendTime/<time>")
                    .delete(get_video_current_tine),
            ),
        )
}
