use clap::{
    app_from_crate, crate_authors, crate_description, crate_name, crate_version, AppSettings,
    SubCommand,
};

fn next_workspace_number(conn: &mut swayipc::Connection) -> Result<i32, swayipc::Error> {
    let workspaces = conn.get_workspaces()?;
    let mut ids: Vec<i32> = workspaces.iter().map(|w| w.num).collect();
    ids.sort_unstable();
    Ok(ids
        .windows(2)
        .find(|w| w[0] + 1 != w[1])
        .map_or(ids.len() as i32 + 1, |w| w[0] + 1))
}

fn main() -> Result<(), swayipc::Error> {
    let params = app_from_crate!()
        .about("A command to create new Sway workpaces")
        .subcommand(SubCommand::with_name("open").about("Open a new workspace"))
        .subcommand(
            SubCommand::with_name("move").about("Move the current container to a new workspace"),
        )
        .setting(AppSettings::SubcommandRequired)
        .get_matches();
    let mut conn = swayipc::Connection::new()?;
    match params.subcommand_name().unwrap() {
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
