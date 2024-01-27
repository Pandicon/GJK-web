# GJK web API

## Adding routes
There are two main ways of adding new routes - either through creating a new route on the `app` object in `main.rs` directly, or creating a new file in the `src/routes` folder (the build script will then take care of the rest). It is also possible to have files in nested folders (`src/routes/news`) and names do not matter at all (the name just can not be `mod.rs` - those files are ignored as they should only be used to bring the other paths into the Cargo project).
If you choose to add a route by adding a file to the `routes` folder, it has to follow these guidelines:
 - It has to have a constant named `ROUTE` (or `_ROUTE`), which has the value of the route of the route (`const _ROUTE: &'static str = "/news/trending";` will create a route at `/news/trending`).
 - It has to have a constant named `TYPE` (or `_TYPE`), which has the value of the type of the route, either `GET` or `POST` (case is ignored, can be `pOSt` if one really wants) (`const _TYPE: &'static str = "GET";` will create a get route).
 - It has to have a public async function called `callback`, which is the function passed into the axum router, so the parameters and return values just have to be valid for the router.
A very simple example is here (`src/routes/root.rs`):
```
pub const _ROUTE: &'static str = "/";
pub const _TYPE: &'static str = "GET";

pub async fn callback() -> &'static str {
	"Hi"
}
```
This creates a root "get" route which responds with "Hi".

To limit access to the endpoint based on permissions, you can add a `PERMISSIONS` (or `_PERMISSIONS`) constant in the form of `PERMISSION1 | PERMISSION2 | ...` which would lead to the endpoint requiring all the permissions in the string. If the constant is not present, it defaults to an open endpoint (no permissions, same as adding `PERMISSIONS = "NONE"`).

An example, where the user is required to have the `READ_SUBSTITUTIONS` and `READ_SCHEDULES` permissions:
```
pub const _PERMISSIONS: &'static str = "READ_SUBSTITUTIONS | READ_SCHEDULES";
pub const _ROUTE: &'static str = "/";
pub const _TYPE: &'static str = "GET";

pub async fn callback() -> &'static str {
	"Hi"
}
```

## Building and running
The project can be simply ran via `cargo run` or `cargo run --release`. However, you may need to install/build sqlite.

### Building SQLite on Windows
Basically follow [this](https://gist.github.com/zeljic/d8b542788b225b1bcb5fce169ee28c55) gist. Once you build the `.lib` file, copy it into a folder where you will put all other `.lib` files in the future (`C:\libs` for the sake of example). Then create a new environment variable called `LIB` and set it to the path to the folder you just created (for example `C:\libs`). If you already have a folder like this, just put the `sqlite.lib` file there.