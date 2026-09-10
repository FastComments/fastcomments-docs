---
Da biste vidjeli sve widgete i tokove koji se pokreću lokalno protiv javnog `demo` najemnika, klonirajte repozitorij i pokrenite:

```bash
npm install
npm run build
cd examples/example-showcase
npm install
npm run dev
```

Primjeri se povezuju s bibliotekom putem `file:../..`, pa je korak `npm run build` u korijenu potreban jednom da bi se proizveo `dist/`.

Svaki widget/tok ima svoj prikaz pod `examples/example-showcase/src/views/` koji možete izravno kopirati u svoju React aplikaciju.
---