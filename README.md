# Godot Keylogger

This is a stopgap solution for Godot to support Global input handling.  Useful for applications which should be able to read input while in the background.

Currently only supports Linux by means of listening to LibInput/evdev

## Proposal References

- https://github.com/godotengine/godot-proposals/issues/1919
- https://github.com/godotengine/godot-proposals/issues/11607

# Building the Extension

This extension is built using godot-rs
You can build it by using the supplied makefile

```
# makes both debug and release builds
make package
```

When building for linux as the target, you will also need the libinput and udev headers.

eg. in ubuntu

```
apt install libudev-dev libinput-dev
```

**Note: The repository does not include any precompiled binaries.  The demo will not function until you compile the extension yourself**

# Using the Extension

The extension will automatically add GlobalInput singletons upon load.

You can poll GlobalInput with the same enums and similar functions as Input within the _process function and other places.  It updates its state once per process frame.

In this example, we can see our Node printing a different message based on if the `A key` is pressed while the window is in focus or not.
```
extends Node

func _process(delta):
    if Input.is_key_pressed(KEY_A):
        print("hello alice")
    elif GlobalInput.is_key_pressed(KEY_A):
        print("hello bob")
```

Mouse input and Pen Pressure are also exposed, allowing you to always get up to date values from hardware where Godot may be otherwise limited by the windowing system.

## Input is not Detected (Linux)

Some distributions do not have users added to the required libinput group by default for security purposes (prevent keylogging apps).


To fix this, you can sudo with the input group each time you run the app.

```
sudo -g input -u <username> open_vt.x86_64
```

Or you can permanently add the input group to the user you wish to run the application under with usermod.  (This is less secure, but you only have to do it once)

```
sudo usermod -a -G input <username>
```
