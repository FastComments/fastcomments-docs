---
For at se hver widget og flow kørende lokalt mod den offentlige `demo` lejer, klon repoet og kør:

```bash
npm install
npm run build
cd examples/example-showcase
npm install
npm run dev
```

Eksemplerne linker til biblioteket via `file:../..`, så root `npm run build`-trinnet er nødvendigt én gang for at producere `dist/`.

Hver widget/flow har sin egen visning under `examples/example-showcase/src/views/`, som du kan kopiere direkte ind i din egen React-app.
---