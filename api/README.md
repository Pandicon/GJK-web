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