Om elke widget en flow lokaal te zien draaien tegen de openbare `demo` tenant, kloon de repo en voer uit:

```bash
npm install
npm run build
cd examples/example-showcase
npm install
npm run dev
```

De voorbeelden linken naar de bibliotheek via `file:../..`, dus de root `npm run build` stap is één keer nodig om `dist/` te produceren.

Elke widget/flow heeft zijn eigen view onder `examples/example-showcase/src/views/` die je direct kunt kopiëren naar je eigen React-app.