# Run the App
* Start the native app: `cargo run`
* Start the web build: `trunk serve`
    * requires [trunk]: `cargo install --locked trunk`
    * requires `wasm32-unknown-unknown` target: `rustup target add wasm32-unknown-unknown`
    * this will serve your app on `8080` and automatically rebuild + reload it after code changes

You should keep the `credits` directory up to date. The release workflow automatically includes the directory in every build.

# Deploy web build to GitHub pages
 1. Trigger the `deploy-github-page` workflow
 2. Activate [GitHub pages](https://pages.github.com/) for your repository
     1. Source from the `gh-pages` branch (created by the just executed action)
 3. After a few minutes your game is live at `http://username.github.io/repository`

To deploy newer versions, just run the `deploy-github-page` workflow again.

# Known issues

Audio in web-builds can have issues in some browsers. This seems to be a general performance issue and not due to the audio itself.

# License

* This work is dual licensed under either of following at your option:
    * [MIT License](LICENSE-MIT.md)
    * [Apache License, Version 2.0](LICENSE-APACHE.md)
