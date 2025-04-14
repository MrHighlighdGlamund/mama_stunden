use dioxus::html::g::style;
use dioxus::mobile::Config;
use printpdf::{BuiltinFont, Mm, PdfDocument};
// use jni::{objects::JClass, sys::JNIEnv};
use views::{choose_month, choose_profile, create_entry, main_view};
mod pdf_create;
mod views {
    pub mod choose_month;
    pub mod choose_profile;
    pub mod create_entry;
    pub mod main_view;
}
use chrono::Local;
use jni::objects::{JClass, JObject, JString, JValue};
use jni::sys::jint;
use jni::{JNIEnv, JavaVM};
pub mod database;
use std::{fs::File, io::BufWriter, sync::OnceLock};
use std::{
    cell::RefCell,
    rc::Rc,
    sync::{Arc, Mutex},
    thread,
    time::Duration,
};

use database::State;
use dioxus::prelude::*;
use rusqlite::{params, Connection};

#[derive(Clone, PartialEq)]
pub enum View {
    MainView,
    ChooseMonth,
    CreateEntry,
    ChooseProfile,
}

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/main.css");
const HEADER_SVG: Asset = asset!("/assets/header.svg");

fn main() {


    insert();

    initialize_channels();
    let mut config = Config::default();
    let config = config.with_custom_head("<style>body { background-color: #E0A0B0; margin: 0; padding: 0;font-size: 16px; font-family: 'Segoe UI', Tahoma, Geneva, Verdana, sans-serif;
; }</style>".to_string());
    let launcher = LaunchBuilder::new();
    launcher.with_cfg(config).launch(App);
    // dioxus::launch(App);
}

#[component]
fn App() -> Element {
    let mut state = State::new(CHANNEL_TO_GUI.get().unwrap().0.clone());
    // state.get_first_month_of_person();
    state.get_last_month_with_entries();
    use_context_provider(|| state);

    chooser()
}
fn chooser() -> Element {
    let view = use_signal(|| View::MainView);
    rsx! {
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        match *view.read() {
            View::ChooseMonth => rsx! { choose_month::choose_month { view } },
            View::CreateEntry => rsx! { create_entry::create_entry { view} },
            View::MainView => rsx! { main_view::main_view { view} },
            View::ChooseProfile => rsx! { choose_profile::choose_profile { view} },
        }
        // }
    }
}
fn insert() {
    let db_path = "/data/data/com.sohnidas_studios.boonk_arbeitszeit/databases/database.db";
    // check if file exists
    if std::path::Path::new(db_path).exists() {
        return;
        // delete file
        // std::fs::remove_file(db_path).unwrap();
    }

    std::fs::create_dir_all("/data/data/com.sohnidas_studios.boonk_arbeitszeit/databases").unwrap();

    // Create or open the SQLite database
    let conn = Connection::open(db_path).unwrap();

    // Optionally create a table
    conn.execute_batch(
        "
CREATE TABLE persons (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL UNIQUE,  
    expectedHours REAL DEFAULT 200.0
);

CREATE TABLE months (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    person_id INTEGER NOT NULL,
    month TEXT NOT NULL,
    overtimeBalance REAL DEFAULT 0.0,
    year INTEGER NOT NULL,
    UNIQUE(person_id, month, year),  
    FOREIGN KEY (person_id) REFERENCES persons(id) ON DELETE CASCADE
);

CREATE TABLE entries (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    month_id INTEGER NOT NULL,
    date TEXT NOT NULL,  
    text TEXT NOT NULL,
    hours REAL NOT NULL,
    FOREIGN KEY (month_id) REFERENCES months(id) ON DELETE CASCADE
);

CREATE TRIGGER update_overtime_balance_after_insert
AFTER INSERT ON entries
FOR EACH ROW
BEGIN
    UPDATE months
    SET overtimeBalance = (
        (
            (SELECT IFNULL(SUM(hours), 0) FROM entries WHERE month_id = NEW.month_id) 
            + (SELECT IFNULL(overtimeBalance, 0) FROM months WHERE id = NEW.month_id)
            - (SELECT expectedHours FROM persons 
               WHERE id = (SELECT person_id FROM months WHERE id = NEW.month_id))
        )
    )
    WHERE id = NEW.month_id + 1;
END;
CREATE TRIGGER update_overtime_balance_after_delete
AFTER DELETE ON entries
FOR EACH ROW
BEGIN
    UPDATE months
    SET overtimeBalance = (
        (
            -- Sum of all hours worked in the current month excluding the deleted entry
            (SELECT IFNULL(SUM(hours), 0) FROM entries WHERE month_id = OLD.month_id)
            -- Add the current month's overtime balance
            + (SELECT IFNULL(overtimeBalance, 0) FROM months WHERE id = OLD.month_id)
            -- Subtract the expected hours for the person
            - (SELECT expectedHours FROM persons 
               WHERE id = (SELECT person_id FROM months WHERE id = OLD.month_id))
        )
    )
    WHERE id = OLD.month_id + 1;
END;





",
    )
    .unwrap(); // Insert persons
    let persons = ["Gaby Katier", "Hermann Katier"];
    let mut person_ids = Vec::new();
    let mut expected_hours = 200.0;

    for name in persons.iter() {
        conn.execute("INSERT INTO persons (name) VALUES (?1)", params![name])
            .unwrap();

        person_ids.push(conn.last_insert_rowid());
        conn.execute(
            "UPDATE persons SET expectedHours = ?1 WHERE name = ?2",
            params![expected_hours, name],
        )
        .unwrap();
    }
    let mut months: Vec<String> = Vec::new();
    months.push(String::from("Januar"));
    months.push(String::from("Februar"));
    months.push(String::from("Maerz"));
    months.push(String::from("April"));
    months.push(String::from("Mai"));
    months.push(String::from("Juni"));
    months.push(String::from("Juli"));
    months.push(String::from("August"));
    months.push(String::from("September"));
    months.push(String::from("Oktober"));
    months.push(String::from("November"));
    months.push(String::from("Dezember"));

    for person_id in person_ids.iter() {
        for i in 2025..2050 {
            for month in months.iter() {
                conn.execute(
                    "INSERT INTO months (person_id, month, year) VALUES (?1, ?2, ?3)",
                    params![person_id, month, i],
                )
                .unwrap();
            }
        }
    }
}
fn initialize_channels() {
    if CHANNEL_TO_GUI.set(crossbeam_channel::unbounded()).is_ok() {}
}
// static mut JVM: Option<JavaVM> = None;
static CHANNEL_TO_GUI: OnceLock<(
    crossbeam_channel::Sender<String>,
    crossbeam_channel::Receiver<String>,
)> = OnceLock::new();
#[no_mangle]
pub extern "system" fn Java_dev_dioxus_main_RustBridge_callRustFunction(
    env: JNIEnv,
    class: JClass,
) {
    let reciver = CHANNEL_TO_GUI.get().unwrap().1.clone();

    loop {
        thread::sleep(Duration::from_millis(50));
        match reciver.try_recv() {
            Ok(file_path) => {


                let context = env
                    .call_static_method(
                        "android/app/ActivityThread",
                        "currentApplication",
                        "()Landroid/app/Application;",
                        &[],
                    )
                    .unwrap()
                    .l()
                    .unwrap();

                // Step 2: Check if the READ_EXTERNAL_STORAGE permission is granted
                let permission_result = env
                    .call_method(
                        context,
                        "checkSelfPermission",
                        "(Ljava/lang/String;)I",
                        &[env
                            .new_string("android.permission.MANAGE_EXTERNAL_STORAGE")
                            .unwrap()
                            .into()],
                    )
                    .unwrap()
                    .i()
                    .unwrap();

                if permission_result != 0 {
                    // Permission not granted, delegate the request to Java/Kotlin
                    println!("Permission not granted for MANAGE_EXTERNAL_STORAGE");

                    // Get the method ID for the requestStoragePermission method
                } else {
                    // Permission granted, proceed with your logic
                    println!("Permission granted for MANAGE_EXTERNAL_STORAGE");
                }
                // let file_path = "/storage/emulated/0/Download/test.pdf"; // Change as needed

                // Convert the file path into a Java String for File object
                let file_path_java = env
                    .new_string(file_path)
                    .expect("Couldn't create Java string");

                // Get the File object for the provided file path
                let file_class = env.find_class("java/io/File").unwrap();
                let file_obj = env
                    .new_object(
                        file_class,
                        "(Ljava/lang/String;)V",
                        &[file_path_java.into()],
                    )
                    .unwrap();

                // Get the application context (current application)
                let context = env
                    .call_static_method(
                        "android/app/ActivityThread",
                        "currentApplication",
                        "()Landroid/app/Application;",
                        &[],
                    )
                    .unwrap()
                    .l()
                    .unwrap();

                // Define the provider authority (must match the one in AndroidManifest)
                let provider_auth = env.new_string("dev.dioxus.main.provider").unwrap();

                // Get the FileProvider class and obtain the URI for the file
                let file_provider_class = env
                    .find_class("androidx/core/content/FileProvider")
                    .unwrap();
                let uri = env
            .call_static_method(
                file_provider_class,
                "getUriForFile",
                "(Landroid/content/Context;Ljava/lang/String;Ljava/io/File;)Landroid/net/Uri;",
                &[context.into(), provider_auth.into(), file_obj.into()],
            )
            .unwrap()
            .l()
            .unwrap();

                // Create the Intent object
                let intent_class = env.find_class("android/content/Intent").unwrap();
                let intent = env.new_object(intent_class, "()V", &[]).unwrap();

                // Set the action for the intent (sending data)
                let action_send = env.new_string("android.intent.action.SEND").unwrap();
                env.call_method(
                    intent,
                    "setAction",
                    "(Ljava/lang/String;)Landroid/content/Intent;",
                    &[action_send.into()],
                )
                .unwrap();

                // Set the MIME type to "application/pdf" for sending PDF files
                let pdf_type = env.new_string("application/pdf").unwrap();
                env.call_method(
                    intent,
                    "setType",
                    "(Ljava/lang/String;)Landroid/content/Intent;",
                    &[pdf_type.into()],
                )
                .unwrap();

                // Attach the URI as the extra to the intent (the file to share)
                env.call_method(
                    intent,
                    "putExtra",
                    "(Ljava/lang/String;Landroid/os/Parcelable;)Landroid/content/Intent;",
                    &[
                        env.new_string("android.intent.extra.STREAM")
                            .unwrap()
                            .into(),
                        uri.into(),
                    ],
                )
                .unwrap();

                // Grant temporary read permission to the URI
                let flag_grant_persistable_uri_permission = 1 << 2; // Intent.FLAG_GRANT_PERSISTABLE_URI_PERMISSION
                let flag_grant_prefix_uri_permission = 1 << 3; // Intent.FLAG_GRANT_PREFIX_URI_PERMISSION

                let full_permissions = flag_grant_prefix_uri_permission
                    | flag_grant_persistable_uri_permission
                    | flag_grant_prefix_uri_permission;
                env.call_method(
                    intent,
                    "addFlags",
                    "(I)Landroid/content/Intent;",
                    &[full_permissions.into()],
                )
                .unwrap();

                // Create a chooser (this will allow the user to pick which app to use for sharing)
                let chooser_title = env.new_string("Share PDF via").unwrap();
                let chooser = env
            .call_static_method(
                intent_class,
                "createChooser",
                "(Landroid/content/Intent;Ljava/lang/CharSequence;)Landroid/content/Intent;",
                &[intent.into(), chooser_title.into()],
            )
            .unwrap()
            .l()
            .unwrap();

                // Add flags for starting the chooser activity
                let flag_activity_new_task = 0x10000000; // Intent.FLAG_ACTIVITY_NEW_TASK
                env.call_method(
                    chooser,
                    "addFlags",
                    "(I)Landroid/content/Intent;",
                    &[flag_activity_new_task.into()],
                )
                .unwrap();

                // Start the activity (open the sharing chooser)
                env.call_method(
                    context,
                    "startActivity",
                    "(Landroid/content/Intent;)V",
                    &[chooser.into()],
                )
                .unwrap();
            }
            _ => {}
        }
    }

    // INIT.call_once(|| {
    //     unsafe {
    //         JVM = Some(vm);
    //     }
    // });

    // call_share_pdf_from_rust();
    // let context = env
    //     .call_static_method(
    //         "android/app/ActivityThread",
    //         "currentApplication",
    //         "()Landroid/app/Application;",
    //         &[],
    //     )
    //     .unwrap()
    //     .l()
    //     .unwrap();

    // // Step 2: Check if the READ_EXTERNAL_STORAGE permission is granted
    // let permission_result = env
    //     .call_method(
    //         context,
    //         "checkSelfPermission",
    //         "(Ljava/lang/String;)I",
    //         &[env
    //             .new_string("android.permission.MANAGE_EXTERNAL_STORAGE")
    //             .unwrap()
    //             .into()],
    //     )
    //     .unwrap()
    //     .i()
    //     .unwrap();

    // if permission_result != 0 {
    //     // Permission not granted, delegate the request to Java/Kotlin
    //     println!("Permission not granted for MANAGE_EXTERNAL_STORAGE");

    //     // Get the method ID for the requestStoragePermission method
    // } else {
    //     // Permission granted, proceed with your logic
    //     println!("Permission granted for MANAGE_EXTERNAL_STORAGE");
    // }
    // let file_path = "/storage/emulated/0/Download/test.pdf"; // Change as needed

    // // Convert the file path into a Java String for File object
    // let file_path_java = env
    //     .new_string(file_path)
    //     .expect("Couldn't create Java string");

    // // Get the File object for the provided file path
    // let file_class = env.find_class("java/io/File").unwrap();
    // let file_obj = env
    //     .new_object(
    //         file_class,
    //         "(Ljava/lang/String;)V",
    //         &[file_path_java.into()],
    //     )
    //     .unwrap();

    // // Get the application context (current application)
    // let context = env
    //     .call_static_method(
    //         "android/app/ActivityThread",
    //         "currentApplication",
    //         "()Landroid/app/Application;",
    //         &[],
    //     )
    //     .unwrap()
    //     .l()
    //     .unwrap();

    // // Define the provider authority (must match the one in AndroidManifest)
    // let provider_auth = env.new_string("com.example.mutti.provider").unwrap();

    // // Get the FileProvider class and obtain the URI for the file
    // let file_provider_class = env
    //     .find_class("androidx/core/content/FileProvider")
    //     .unwrap();
    // let uri = env
    //     .call_static_method(
    //         file_provider_class,
    //         "getUriForFile",
    //         "(Landroid/content/Context;Ljava/lang/String;Ljava/io/File;)Landroid/net/Uri;",
    //         &[context.into(), provider_auth.into(), file_obj.into()],
    //     )
    //     .unwrap()
    //     .l()
    //     .unwrap();

    // // Create the Intent object
    // let intent_class = env.find_class("android/content/Intent").unwrap();
    // let intent = env.new_object(intent_class, "()V", &[]).unwrap();

    // // Set the action for the intent (sending data)
    // let action_send = env.new_string("android.intent.action.SEND").unwrap();
    // env.call_method(
    //     intent,
    //     "setAction",
    //     "(Ljava/lang/String;)Landroid/content/Intent;",
    //     &[action_send.into()],
    // )
    // .unwrap();

    // // Set the MIME type to "application/pdf" for sending PDF files
    // let pdf_type = env.new_string("application/pdf").unwrap();
    // env.call_method(
    //     intent,
    //     "setType",
    //     "(Ljava/lang/String;)Landroid/content/Intent;",
    //     &[pdf_type.into()],
    // )
    // .unwrap();

    // // Attach the URI as the extra to the intent (the file to share)
    // env.call_method(
    //     intent,
    //     "putExtra",
    //     "(Ljava/lang/String;Landroid/os/Parcelable;)Landroid/content/Intent;",
    //     &[
    //         env.new_string("android.intent.extra.STREAM")
    //             .unwrap()
    //             .into(),
    //         uri.into(),
    //     ],
    // )
    // .unwrap();

    // // Grant temporary read permission to the URI
    // let flag_grant_persistable_uri_permission = 1 << 2; // Intent.FLAG_GRANT_PERSISTABLE_URI_PERMISSION
    // let flag_grant_prefix_uri_permission = 1 << 3; // Intent.FLAG_GRANT_PREFIX_URI_PERMISSION

    // let full_permissions = flag_grant_prefix_uri_permission
    //     | flag_grant_persistable_uri_permission
    //     | flag_grant_prefix_uri_permission;
    // env.call_method(
    //     intent,
    //     "addFlags",
    //     "(I)Landroid/content/Intent;",
    //     &[full_permissions.into()],
    // )
    // .unwrap();

    // // Create a chooser (this will allow the user to pick which app to use for sharing)
    // let chooser_title = env.new_string("Share PDF via").unwrap();
    // let chooser = env
    //     .call_static_method(
    //         intent_class,
    //         "createChooser",
    //         "(Landroid/content/Intent;Ljava/lang/CharSequence;)Landroid/content/Intent;",
    //         &[intent.into(), chooser_title.into()],
    //     )
    //     .unwrap()
    //     .l()
    //     .unwrap();

    // // Add flags for starting the chooser activity
    // let flag_activity_new_task = 0x10000000; // Intent.FLAG_ACTIVITY_NEW_TASK
    // env.call_method(
    //     chooser,
    //     "addFlags",
    //     "(I)Landroid/content/Intent;",
    //     &[flag_activity_new_task.into()],
    // )
    // .unwrap();

    // // Start the activity (open the sharing chooser)
    // env.call_method(
    //     context,
    //     "startActivity",
    //     "(Landroid/content/Intent;)V",
    //     &[chooser.into()],
    // )
    // .unwrap();
}
