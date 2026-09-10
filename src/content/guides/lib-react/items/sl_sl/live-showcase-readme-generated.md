---
Če želite videti vsak pripomoček in tok, ki tečejo lokalno proti javnemu najemniku `demo`, klonirajte repozitorij in zaženite:

```bash
npm install
npm run build
cd examples/example-showcase
npm install
npm run dev
```

Primeri se povežejo s knjižnico prek `file:../..`, zato je korak `npm run build` v korenu potreben enkrat, da se ustvari `dist/`.

Vsak pripomoček/tok ima svoj pogled v `examples/example-showcase/src/views/`, ki ga lahko neposredno kopirate v svojo React aplikacijo.
---