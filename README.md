# Hypr Wide

A program which resizes hyprland windows to 16:9 when alone on a widescreen monitor.

## The inner workings

- When spawning a new window check if the workspace is empty
- If the workspace is empty, set the window to pseudotiled and set the size to _roughly_ 16:9
- Save the id of the resized window
- If a new window is spawned (or moved) to the workspace set the original window to tiled
- If the user manually changes the tiling of the saved window the window should be deleted from the managed list
- Need a way to mark a current window back into the managed list
