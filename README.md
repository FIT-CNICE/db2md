## Dev
First, install tailwindcss and Dioxus CLI:
```shell
cargo install dioxus-cli
npm install -g tailwindcss
```
Open two terminal panes and run the following:

```shell
npx tailwindcss -i ./input.css -o assets/tailwind.css --watch
```

```shell
dx serve
```
