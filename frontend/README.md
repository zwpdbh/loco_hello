# Development

1. Install npm: https://docs.npmjs.com/downloading-and-installing-node-js-and-npm
2. Install the tailwind css cli: https://tailwindcss.com/docs/installation
3. Run the following command in the root of the project to start the tailwind CSS compiler:

```bash
npx tailwindcss -i ./input.css -o ./assets/tailwind.css --watch
```

Run the following command in the root of the project to start the Dioxus dev server:

```bash
dx serve --hot-reload
```

- Open the browser to http://localhost:8080


## How to install tailwind css

- Install npx 

```sh
curl -o- https://raw.githubusercontent.com/nvm-sh/nvm/v0.39.1/install.sh | bash
source ~/.bashrc
nvm list-remote

nvm install <version>
```

- Generate tailwind.css file 

Based on the `Dioxus.toml` file definition for css 
``` toml 
style = ["/tailwind.css"]
```

Generate the tailwind.css file given `tailwind.config.js` and `input.css` is prepared from Dioxus, we only need to run: 

```sh
npx tailwindcss -i ./input.css -o ./tailwind.css --watch
```