use crate::config::mysql::RB;
use crate::config::redis::{redis_delete, redis_read, redis_write, redis_write_and_rm};
use crate::pojo::token::Token;
use crate::pojo::users::Users;
use crate::res::result::ResponseData;
use crate::services::users_service::UsersService;
use salvo::prelude::*;
use salvo::{Request, Response};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize, Extractible, Clone)]
struct UserInfo {
    username: String,
    password: String,
}

// 确保这个结构体实现了 Serialize 特性
#[derive(Debug, Serialize, Deserialize, Extractible, Clone)]
struct UserData {
    username: String,
    roles: Vec<String>,
}

pub struct UsersServicesImpl;
impl UsersService for UsersServicesImpl {
    async fn login_post(req: &mut Request, res: &mut Response) {
        //示例：http://127.0.0.1:5800/login/?username=admin&password=123456
        let user_info = req.parse_json::<UserInfo>().await;
        println!("{:?}", &user_info);
        let user_info = user_info.unwrap();
        let username = user_info.username;
        let password = user_info.password;
        let data = Box::new(
            Users::login(&RB.clone(), username.to_string(), password.to_string())
                .await
                .unwrap(),
        );

        if data.is_none() {
            let data: ResponseData<()> = ResponseData::error("用户名或密码错误");
            return res.render(serde_json::to_string(&data).unwrap());
        }

        let rs = redis_write(
            "now_user_role",
            &<Option<Users> as Clone>::clone(&data).unwrap()._type,
        )
            .await;

        if rs.is_err() {
            let data: ResponseData<()> = ResponseData::error("Redis连接错误");
            return res.render(serde_json::to_string(&data).unwrap());
        }

        let rs = redis_write(
            "now_user_name",
            &<Option<Users> as Clone>::clone(&data).unwrap().username,
        )
            .await;

        if rs.is_err() {
            let data: ResponseData<()> = ResponseData::error("Redis连接错误");
            return res.render(serde_json::to_string(&data).unwrap());
        }
        let now_token = Token {
            token: format!(
                "token-{}",
                &*<Option<Users> as Clone>::clone(&data).unwrap()._type
            ),
        };
        let data = ResponseData::success(now_token, "登录成功");

        println!("{:?}", &data);
        res.render(serde_json::to_string(&data).unwrap())
    }

    async fn users_info(res: &mut Response) {
        let roles = redis_read("now_user_role").await.unwrap_or_default();
        let roles = vec![roles];
        let username = redis_read("now_user_name").await.unwrap_or_default();
        let rs_data = UserData {
            username,
            roles: roles.clone(),
        };
        let data =
            ResponseData::success(rs_data, &("获取".to_owned() + &roles[0] + "类型用户成功"));
        res.render(serde_json::to_string(&data).unwrap());
    }

    async fn users_logout(res: &mut Response) {
        let rs = redis_delete("now_user_role").await;
        if rs.is_err() {
            let data: ResponseData<()> = ResponseData::error("Redis链接有误");
            res.render(serde_json::to_string(&data).unwrap())
        }
        let rs = redis_delete("now_user_name").await;
        if rs.is_err() {
            let data: ResponseData<()> = ResponseData::error("Redis链接有误");
            res.render(serde_json::to_string(&data).unwrap())
        }
        let rs = ResponseData::success("", "清除Redis缓存成功");
        res.render(serde_json::to_string(&rs).unwrap())
    }

    async fn users_get_all(req: &mut Request, res: &mut Response) {
        let users_list_key = "usersList";

        let rs = redis_read(users_list_key).await;
        if rs.is_err() {
            println!("redis无数据");
            //从数据库拿取用户列表
            let user_data = Box::new(Users::get_all_user(&RB.clone()).await.unwrap());
            //写入redis
            let serialized = serde_json::to_string(&user_data).unwrap();
            let rs = redis_write_and_rm(users_list_key, &serialized, 1).await;
            if rs.is_err() {
                eprintln!("Redis操作有误");
            }
            //---
        }

        //从redis拿取用户数据
        let rs = redis_read(users_list_key).await;
        let rs: &str = &rs.unwrap();
        let users: Vec<Users> = serde_json::from_str(rs).unwrap();

        //分页处理
        let total = users.len();
        //获取请求中的当前页以及一页的数量
        let current_page = req.query::<String>("currentPage").unwrap_or_default();
        let size = req.query::<String>("size").unwrap_or_default();

        //类型转换
        let current_page = current_page.parse::<usize>().unwrap();
        let size = size.parse::<usize>().unwrap();
        let rs = format!("Current Page: {}, Size: {}", &current_page, &size);
        println!("{}", rs);

        //计算显示的索引范围
        let mut get_end_index = current_page * size;
        let get_start_index = get_end_index - size + 1;
        if get_end_index > total {
            get_end_index = total;
        }

        let users: Vec<Users> = users[get_start_index - 1..get_end_index].to_owned();

        let mut data: HashMap<&str, Vec<Users>> = HashMap::new();
        data.insert("list", users);
        let data = ResponseData::success(data, "成功");
        res.render(serde_json::to_string(&data).unwrap())
    }
}
