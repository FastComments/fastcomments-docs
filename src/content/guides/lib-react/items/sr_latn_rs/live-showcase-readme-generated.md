---
Da biste videli svaki widget i flow koji se pokreću lokalno protiv javnog `demo` tenant-a, klonirajte repo i pokrenite:

```bash
npm install
npm run build
cd examples/example-showcase
npm install
npm run dev
```

Primeri se povezuju sa bibliotekom putem `file:../..`, tako da je korak `npm run build` na korenu potreban jednom da bi se proizveo `dist/`.

Svaki widget/flow ima svoj prikaz pod `examples/example-showcase/src/views/` koji možete direktno kopirati u svoju React aplikaciju.
---