use super::resolve_next_workspace_number;

#[test]
fn no_workspaces() {
    assert_eq!(
        resolve_next_workspace_number([].to_vec()),
        1
    );
}

#[test]
fn negative_workspaces() {
    assert_eq!(
        resolve_next_workspace_number([-1].to_vec()),
        1
    );
}

#[test]
fn mixed_negative_and_positive_workspaces() {
    assert_eq!(
        resolve_next_workspace_number([-1, 1, 2, -1, 3, -1, -1].to_vec()),
        4
    );
}

#[test]
fn consecutive() {
    assert_eq!(
        resolve_next_workspace_number([1, 2, 3, 4].to_vec()),
        5
    );
}

#[test]
fn non_consecutive() {
    assert_eq!(
        resolve_next_workspace_number([4, 1, 3, 2].to_vec()),
        5
    );
}

#[test]
fn missing_1() {
    assert_eq!(
        resolve_next_workspace_number([4, 3, 2].to_vec()),
        1
    );
}

#[test]
fn missing_2() {
    assert_eq!(
        resolve_next_workspace_number([4, 3, 1].to_vec()),
        2
    );
}

#[test]
fn duplicates_begin() {
    assert_eq!(
        resolve_next_workspace_number([1, 1, 1, 2, 3, 4].to_vec()),
        5
    );
}

#[test]
fn duplicates_end() {
    assert_eq!(
        resolve_next_workspace_number([1, 2, 3, 4, 4, 4].to_vec()),
        5
    );
}

#[test]
fn duplicates_middle() {
    assert_eq!(
        resolve_next_workspace_number([1, 2, 3, 3, 3, 4].to_vec()),
        5
    );
}

#[test]
fn duplicate_and_gaps() {
    assert_eq!(
        resolve_next_workspace_number([5, 9, 4, 10, 1, 2, 3, 4, 5, 7].to_vec()),
        6
    );
}
