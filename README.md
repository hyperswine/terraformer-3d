# Terraformer Engine
A 3D game engine but completely in rust/std/cargo.
- based on godot 3d and unreal 4
- technically platform independent given the no problems in the underlying dependencies. But optimised unsafe blocks for Quanta.
- rn, just the backend. Can build a frontend in iced-rs that directly connects to the backend.

## Key features
- make interactive 3D first person/3rd person games easily
- range of tools like AI pathfinding, collision, physics, etc. built in
- range of libraries for common models and procedurally generated level with AI
- code in rust, no need for scripting in another language. Once you know rust, you can do anything on Quanta

## Features that sets this apart
- Modularisation with AI modules. Allowing AI.
- Quanta optimised. Dont use some other random engine, just use this one.
- No royalties. Do whatever you want with it. Use it for a hobby, legit game and modify it to your wishes.
- Strong modding and extension features. Like blender and a browser, built in support for modularisation and plugins for niche and cool features. Can sell a plugin if you want since the plugin API and libraries are not GPL licensed.
- If you want, can make a UI in rei/ffx and hook into the vulkan display. Builtin tools to do this.

## Game 1: Demens
Built using the terraformer engine.
- A "pseudo turn based" backyard monsters + civ6 style game where you build bases, expand into the map unknowns, conquer enemies and establish your grand empire of the sun.
- Demens and enemies based on sci-fi eletian soldiers and monsters. Angelic, demonic, entities like in Doom.
- Top-down, 3d based enemies like backyard monsters and civ. Procedurally generated terrain using the terraformer generation engine.

## Game 2: Eletian Journey
A journey through the Veritas of Eletei.
- A pseudo turn based game akin to persona 5, heavy rain and story driven adventure JRPG games.
- AI assisted development using terraformer ML.

## Game 3: Gantz 3D
A VR Gantz-style game where you can be "tasked" at anytime. Put on your spectro VR goggles and jump into a game. Team up with other human or AI players against human or AI aliens!
- A lot of progression. Get 100 points and level up. Then choose a bonus. Either a new weapon or increase in a specific set of stats or new armor.
- AI driven alien generation and combat using terraformer combat AI.

## Dev Philosophy
- code it when you need it, refactor when things get messy and hard to work with. Refactor until it works again and the tests pass
- just get it working, doesnt matter if its inefficient or bad code. Modularise the components and make use of loose coupling to easily refactor and upgrade certain parts
