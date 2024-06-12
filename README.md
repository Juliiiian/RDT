# RDT: Rust-Dev-Tool

## Whats the goal?

Have a little program that copies your files like dll, plugins into the server files for developers. But also provides a better way of running the server with a settings panel, server statistics, etc.

## Long time goals

-   [ ] Custom file copy for plugins and dll
-   [ ] Handle the rust server in rust
-   [ ] Automated extension build (stop server, build, copy, start server)
-   [ ] Update and download the server files through steamcmd
-   [ ] Option to add oxide/carbon
-   [ ] maybe option to add plugins from oxide marketplace (call them by api or do the forbidden scraping)

## Next todos

-   [ ] handle rust server in rust
    -   problem is that the stdout cant read the stuff after initial console fill (ig fp uses some win api, since they also have sever stats later at runtime)
-   [ ] creating main ui (header and co)
-   [ ] basic copy machanic
