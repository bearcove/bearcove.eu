# bearcove.eu

The [bearcove](https://bearcove.eu) website. Built with [dodeca](https://dodeca.bearcove.eu/), deployed to GitHub Pages.

## Development

```bash
ddc serve
```

## Build

```bash
ddc build
```

Output goes to `public/`.

## Site structure

The homepage lives in `content/_index.md`. Research, Products, Publishing, Company,
and Legal notice each have their own content file and route. Shared navigation,
footer, metadata, and the clipboard status region live in `templates/base.html`.
The approved responsive design is in `sass/main.scss`; copy controls are in
`static/site.js`.

Jost is self-hosted under `static/fonts/`, with its SIL Open Font License alongside
it. `jost.bin` is the original WOFF2 file: the neutral extension prevents dodeca
0.17.0 from subsetting away its tabular-digit features. The `@font-face` rule
explicitly declares `format('woff2')`.

## Deployment

The Website GitHub Actions workflow builds with dodeca **0.17.0** and deploys to
GitHub Pages on relevant pushes to `main`. `CNAME` keeps the custom domain
`bearcove.eu`. For a release check, use the same dodeca version as the workflow.
