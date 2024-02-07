use rocket::fairing::AdHoc;

pub(crate) mod resource;
pub(crate) mod user;
pub(crate) mod setting;
pub(crate) mod filebrowser;
pub(crate) mod entity;
pub(crate) mod actions;

pub(crate) fn stage() -> AdHoc {
    AdHoc::on_ignite("api stage", |rocket| async {
        rocket.attach(user::stage())
            .attach(setting::stage())
            .attach(resource::stage())
            .attach(filebrowser::stage())
            .attach(actions::stage())
    })
}
