use std::cmp::Ordering;

use rocket::fairing::AdHoc;
use rocket::serde::json::Json;

use soda_resource_tools_lib::soda;
use soda_resource_tools_lib::soda::filebrowser::FileItem;

use crate::api::entity::Response;

#[get("/api/filebrowser/list?<path>&<sort>")]
pub(crate) async fn api_filebrowser_list(path: String, sort: String) -> Json<Response<Vec<FileItem>>> {
    let decode_path = urlencoding::decode(&path).unwrap().to_string();

    //
    let mut ret_items: Vec<FileItem> = if path.is_empty() {
        soda::filebrowser::list_root_path()
    } else {
        soda::filebrowser::list_sub_path(decode_path, vec![])
    };

    //
    if sort == "name" {
        ret_items.sort_by(|a, b| a.name.partial_cmp(&b.name).unwrap());
    } else if sort == "name_desc" {
        ret_items.sort_by(|a, b| a.name.partial_cmp(&b.name).unwrap());
    } else if sort == "name_asc" {
        ret_items.sort_by(|a, b| b.name.partial_cmp(&a.name).unwrap());
    } else if sort == "time" {
        ret_items.sort_by(|a, b| a.modify_time.partial_cmp(&b.modify_time).unwrap());
    } else if sort == "time_desc" {
        ret_items.sort_by(|a, b| a.modify_time.partial_cmp(&b.modify_time).unwrap());
    } else if sort == "time_asc" {
        ret_items.sort_by(|a, b| b.modify_time.partial_cmp(&a.modify_time).unwrap());
    } else if sort == "type_desc" {
        ret_items.sort_by(|a, b| if a.file_type == "dir" { Ordering::Less } else { Ordering::Greater });
    } else if sort == "type_asc" {
        ret_items.sort_by(|a, b| if a.file_type == "dir" { Ordering::Greater } else { Ordering::Less });
    } else if sort == "name_desc_and_type_desc" {
        ret_items.sort_by(|a, b| {
            // 首先按照文件类型进行比较
            let a_is_dir = a.file_type == "dir";
            let b_is_dir = b.file_type == "dir";
            if a_is_dir && !b_is_dir {
                Ordering::Less
            } else if !a_is_dir && b_is_dir {
                Ordering::Greater
            } else {
                a.name.partial_cmp(&b.name).unwrap()
            }
        });
    }

    return Response::success_to_json(ret_items);
}

pub fn stage() -> AdHoc {
    AdHoc::on_ignite("api filebrowser stage", |rocket| async {
        rocket.mount("/", routes![api_filebrowser_list])
    })
}
