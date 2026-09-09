---
Per vedere ogni widget e flusso in esecuzione localmente contro il tenant pubblico `demo`, clona il repository e esegui:

```bash
npm install
npm run build
cd examples/example-showcase
npm install
npm run dev
```

Gli esempi collegano la libreria tramite `file:../..`, quindi il passaggio radice `npm run build` è necessario una volta per produrre `dist/`.

Ogni widget/flow ha la propria vista sotto `examples/example-showcase/src/views/` che puoi copiare direttamente nella tua app React.
---