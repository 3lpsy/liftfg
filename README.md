# FG Lift

A cross-platform weight lifting applicaiton written in Rust and with `Tauri` and `Dioxus`.

# Status
This project is a WIP. Very little of the UI has been built out but a lot of the backend foundation now exists to focus on the actual application and IPC calls.

# Purpose
I needed a reason to get better at `Rust` and was bored. `Dioxus` seemed fun but I wasn't confident about its cross-platform capabilities so I wrapped it in `Tauri`.

# Idea
While many applications exist to track workouts, most are proscriptive in the specific exercises you do for any given workout. This can make tracking difficult if you workout in different locations where your options of exercises may differ or if the equipment is taken. Therefore, FGLift focuses on which muscles you wish to target for each lift and the desired volume. Instead of having a program that says do `X` number of sets for `Y` exercise, programs simply have target muscle groups with a desired number of sets for each. When you start a workout, you enter your location, and it prompts you for an exercise targeting a given muscle group that you can either skip, for example if the machine is taken and you wish to do an alternative one, or let the application know that the exercise is not available at that location. As you do more workouts, the application will prompt you less and less for exercises that you can't do, but also still allow you to introduce variance by doing alternative exercises.

# Crates
- `fgshell`
  - Contains `Tauri` specific code. Wraps the application and proxies API calls and platform specific operations to the appropriate target. Additioanlly ensures database and logging are in a ready state. Uses `fgcore`, `fgdb`, and `fgutils`.
- `fgcore`
  - Serves as the brain of the application, makes decisions, and interacts with database. Uses `fgutils`, `fgdb`
- `fgui`
  - The primary interface of the applicaiton and contains `Dioxus` specific code. Interfaces with the backend (primarily `shell`) to send and receive data. Uses `fgcore`, `fgdb`, and `fgutils`
- `fgdb`
  - A `SeaOrm` based crate that holds entities, migrations, and seeders. For now, is tightly coupled with request and response like types which it is responsible for defining and determining how to convert to and from database types such as models. Uses `fgcore`, `fgutils`
- `fgutils`
  - A crate that holds utility logic that does not depend on any other crate.
- `fgcli`
  - A devlopment only crate with few features which will most likely be deleted or not built out for some time.
