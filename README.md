# sway-new-workspace
This is a small utility to create a new workspace in Sway.

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