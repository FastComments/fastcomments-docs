To see every widget and flow running locally against the public `demo` tenant, clone the repo and run:

```bash
npm install
npm run build
cd examples/example-showcase
npm install
npm run dev
```

The examples link to the library via `file:../..`, so the root `npm run build` step is needed once to produce `dist/`.

Each widget/flow has its own view under `examples/example-showcase/src/views/` that you can copy straight into your own React app.