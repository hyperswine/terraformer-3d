# Design

- based on godot 3d and unreal 4
- technically platform independent given the no problems in the underlying dependencies. But optimised unsafe blocks for Quanta.
- rn, just the backend. Can build a frontend in iced-rs that directly connects to the backend.

## Features that sets this apart

- Modularisation with AI modules. Allowing AI.
- Quanta optimised. Dont use some other random engine, just use this one.
- No royalties. Do whatever you want with it. Use it for a hobby, legit game and modify it to your wishes.
- Strong modding and extension features. Like blender and a browser, built in support for modularisation and plugins for niche and cool features. Can sell a plugin if you want since the plugin API and libraries are not GPL licensed.
- If you want, can make a UI in rei/ffx and hook into the vulkan display. Builtin tools to do this.

## Solar System Architecture

A solar system contains a central star subsystem, planetary subsystems and asteroid subsystems.

Basically you have a central root that is the causer. And a bunch of subordinates who are affected in someway by the causer's actions, usually in a linear manner.
