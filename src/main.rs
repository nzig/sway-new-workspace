use clap::{command, Command};

fn resolve_next_workspace_number(mut workspaces_number: Vec<i32>) -> i32 {
    workspaces_number.sort_unstable();
    workspaces_number.dedup();
    workspaces_number = workspaces_number
        .into_iter()
        .filter(|&w|  w > 0)
        .collect();

    let len = workspaces_number.len() as i32;
    workspaces_number
        .into_iter()
        .enumerate()
        .find(|&(idx, workspace_num)| idx as i32 + 1 != workspace_num)
        .map_or(len + 1, |(idx, _)| idx as i32 + 1)
}

fn next_workspace_number(conn: &mut swayipc::Connection) -> Result<i32, swayipc::Error> {
    let workspaces = conn.get_workspaces()?;
    Ok(resolve_next_workspace_number(
        workspaces
        .iter()
        .map(|w| w.num)
        .collect()
    ))
}

fn main() -> Result<(), swayipc::Error> {
    let params = command!()
        .about("A command to create new Sway workpaces")
        .subcommand(Command::new("open").about("Open a new workspace"))
        .subcommand(Command::new("move").about("Move the current container to a new workspace"))
        .subcommand_required(true)
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

#[cfg(test)]
mod tests;
