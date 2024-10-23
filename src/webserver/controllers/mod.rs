use actix_web::web;

pub mod authentication_controller;
pub mod user_management_controller;

pub fn config_all_routes(cfg: &mut web::ServiceConfig) {
    authentication_controller::config_routes(cfg);
    user_management_controller::config_routes(cfg);

    cfg.route("/health", web::get().to(|| async { "OK" }));
    // return html
    cfg.route(
        "/",
        web::get().to(|| async {
            r#"
        _____           _                   _   _                _   _           _   _
       |  __ \         | |       /\        | | | |              | | (_)         | | (_)
       | |__) |   _ ___| |_     /  \  _   _| |_| |__   ___ _ __ | |_ _  ___ __ _| |_ _  ___  _ __
       |  _  / | | / __| __|   / /\ \| | | | __| '_ \ / _ \ '_ \| __| |/ __/ _` | __| |/ _ \| '_ \
       | | \ \ |_| \__ \ |_   / ____ \ |_| | |_| | | |  __/ | | | |_| | (_| (_| | |_| | (_) | | | |
       |_|  \_\__,_|___/\__| /_/    \_\__,_|\__|_| |_|\___|_| |_|\__|_|\___\__,_|\__|_|\___/|_| |_|
       "#
        }),
    );
}
