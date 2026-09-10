Um jedes Widget und jeden Flow lokal gegen den öffentlichen `demo`-Mandanten zu sehen, klonen Sie das Repository und führen Sie aus:

```bash
npm install
npm run build
cd examples/example-showcase
npm install
npm run dev
```

Die Beispiele verlinken die Bibliothek über `file:../..`, sodass der Root‑Befehl `npm run build` einmal ausgeführt werden muss, um `dist/` zu erzeugen.

Jedes Widget/Flow hat seine eigene Ansicht unter `examples/example-showcase/src/views/`, die Sie direkt in Ihre eigene React‑App kopieren können.