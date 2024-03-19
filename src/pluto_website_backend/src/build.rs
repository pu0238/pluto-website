use pluto_templating::RucteError;
use std::{
    fs::{copy, create_dir, create_dir_all, read_dir, remove_dir_all, DirEntry, File},
    io::{Read, Write},
    path::Path,
};

fn visit_dirs(
    dir: &Path,
    cb: &dyn Fn(&DirEntry) -> Result<(), RucteError>,
) -> Result<(), RucteError> {
    if dir.is_dir() {
        for entry in read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                visit_dirs(&path, cb)?;
            } else {
                cb(&entry)?;
            }
        }
    }
    Ok(())
}

fn visit_dirs_and_collect_string(
    dir: &Path,
    cb: &dyn Fn(&DirEntry) -> Result<Option<String>, RucteError>,
) -> Result<Vec<String>, RucteError> {
    let mut paths: Vec<String> = Vec::new();
    if dir.is_dir() {
        for entry in read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                let mut vec_paths = visit_dirs_and_collect_string(&path, cb)?;
                paths.append(&mut vec_paths);
            } else {
                if let Some(path) = cb(&entry)? {
                    paths.push(path)
                }
            }
        }
    }

    Ok(paths)
}

fn compiled_paths_to_paths_and_mods(
    paths: Vec<String>,
) -> Result<Vec<(String, String)>, RucteError> {
    let mut routes_and_computed: Vec<(String, String)> = Vec::new();
    let files_to_remove = ["templates", "mod", "_utils", "statics", "router"];

    for path in paths {
        let mut splited_template_files: Vec<String> =
            path.split("/").map(|txt| txt.to_string()).collect();
        if let Some(latest) = splited_template_files.last_mut() {
            if files_to_remove.contains(&latest.as_str()) {
                continue;
            }
            *latest = latest.replace("template_", "");
        }

        let mut splited_route = splited_template_files.clone();
        let remove_to = splited_template_files.len() - 1;
        splited_route = splited_route.drain(1..remove_to).collect();

        let joined_route = splited_route.join("/");
        let path = if joined_route == "" {
            "/".to_string()
        } else {
            "/".to_string() + &joined_route
        };

        routes_and_computed.push((path, splited_template_files.join("::")))
    }

    Ok(routes_and_computed)
}

pub fn main() -> Result<(), RucteError> {
    let views_dir = "views/";
    let static_dir = "static/";
    let astro_out_dir = "../pluto_website_frontend/dist/";

    remove_dir_all(views_dir)?;
    remove_dir_all(static_dir)?;

    let astro_out_path = Path::new(astro_out_dir);
    visit_dirs(astro_out_path, &|entry: &DirEntry| {
        let path = entry.path();
        let extension = path.extension().expect("File do not have extension?");
        let to_remove = astro_out_dir.len();
        let path_to_create = &path.to_str().expect("Not a path?!")[to_remove..];

        if extension == "html" {
            let copy_target = views_dir.to_owned() + path_to_create;
            let copy_target_path = Path::new(&copy_target);

            create_dir_all(copy_target_path.with_file_name(""))?;

            let mut src = File::open(&path)?;
            let mut html_string = String::new();
            src.read_to_string(&mut html_string)?;

            let string_html = html_string.replace("{", "@{").replace("}", "@}");

            let mut created_file = File::create(&copy_target_path.with_extension("rs.html"))?;
            let rust_starter = "@()\n";
            created_file.write_all(rust_starter.as_bytes())?;
            created_file.write_all(string_html.as_bytes())?;
        } else {
            let copy_target = static_dir.to_owned() + path_to_create;
            let copy_target_path = Path::new(&copy_target);

            create_dir_all(copy_target_path.with_file_name(""))?;
            copy(path, copy_target_path)?;
        }
        Ok(())
    })?;

    let out_dir = std::env::var("OUT_DIR").expect("No source path set.");
    let out_path = Path::new(&out_dir);

    pluto_templating::initialize_templating_engine(out_path.to_path_buf(), views_dir, static_dir)?;
    let compiled_paths = visit_dirs_and_collect_string(out_path, &|entry: &DirEntry| {
        let path = entry.path();
        let path_without_extension = path.with_extension("");
        let template_files =
            &path_without_extension.to_str().expect("Not a path?!")[out_dir.len() + 1..];

        Ok(Some(template_files.to_string()))
    })?;

    let mut routes_and_computed = compiled_paths_to_paths_and_mods(compiled_paths)?;
    let mut out_path = out_path.to_path_buf();
    out_path.push("out");

    let mut created_file = File::create(out_path.with_file_name("router.rs"))?;

    let deps = "pub mod router {\n
use std::collections::HashMap;\n
use pluto::{http::HttpRequest, render_view, router::Router};\n\n";
    created_file.write_all(deps.as_bytes())?;

    let router_start = "pub fn setup() -> Router {
    let mut router = Router::new();\n\n";
    created_file.write_all(router_start.as_bytes())?;

    routes_and_computed.sort_by(|(old_path, _), (new_path, _)| {
        new_path.to_lowercase().cmp(&old_path.to_lowercase())
    });
    routes_and_computed.sort_by(|(old_path, _), (new_path, _)| {
        let new_depth: Vec<&str> = new_path.split("/").collect();
        let old_depth: Vec<&str> = old_path.split("/").collect();
        new_depth.len().cmp(&old_depth.len())
    });

    for (path, computed) in routes_and_computed {
        let routers = format!(
            r#"   router.get("{path}", false, |_req: HttpRequest| async move {{
        render_view!(crate::compiled::{computed});
    }});"#
        );
        created_file.write_all(routers.as_bytes())?;
        created_file.write_all("\n\n".as_bytes())?;
    }

    let router_end = "   router
}\n}";
    created_file.write_all(router_end.as_bytes())?;
    Ok(())
}
