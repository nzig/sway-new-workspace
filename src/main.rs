use clap::{app_from_crate, App, AppSettings};

fn next_workspace_number(conn: &mut swayipc::Connection) -> Result<i32, swayipc::Error> {
    let workspaces = conn.get_workspaces()?;
    let mut ids: Vec<i32> = workspaces.iter().map(|w| w.num).collect();
    ids.sort_unstable();
    let len = ids.len() as i32;
    Ok(ids
        .into_iter()
        .enumerate()
        .find(|&(idx, workspace_num)| idx as i32 + 1 != workspace_num)
        .map_or(len + 1, |(idx, _)| idx as i32 + 1))
}

fn main() -> Result<(), swayipc::Error> {
    let params = app_from_crate!()
        .about("A command to create new Sway workpaces")
        .subcommand(App::new("open").about("Open a new workspace"))
        .subcommand(App::new("move").about("Move the current container to a new workspace"))
        .setting(AppSettings::SubcommandRequired)
        .get_matches();
    let mut conn = swayipc::Connection::new()?;
    match params.subcommand_name().expect("unexpected subcommand") {
        "open" => {
            let next_id = next_workspace_number(&mut conn)?;
            conn.run_command(format!("workspace {}", next_id))?;
        }
        "move" => {
            let next_id = next_workspace_number(&mut conn)?;
            conn.run_command(format!("move container to workspace {}", next_id))?;
            conn.run_command(format!("workspace {}", next_id))?;
        }
        _ => unreachable!(),
    }

    Ok(())
}
