# sway-new-workspace &emsp; [![Action Badge]][actions] [![Version Badge]][crates.io]
This is a small utility to create a new workspace in Sway.

[Version Badge]: https://img.shields.io/crates/v/sway-new-workspace.svg
[crates.io]: https://crates.io/crates/sway-new-workspace
[Action Badge]: https://github.com/nzig/sway-new-workspace/workflows/Rust/badge.svg
[actions]: https://github.com/nzig/sway-new-workspace/actions
## Install
```
cargo install sway-new-workspace
```

## Example config
Put this in you sway config (`~/.config/sway/config`)
```
bindsym $mod+n exec sway-new-workspace open
bindsym $mod+Shift+n exec sway-new-workspace move
bindsym $mod+Shift+d exec sway-new-workspace open; exec $menu
```