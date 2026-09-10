---
Para ver cada widget e fluxo sendo executado localmente contra o tenant público `demo`, clone o repositório e execute:

```bash
npm install
npm run build
cd examples/example-showcase
npm install
npm run dev
```

Os exemplos vinculam a biblioteca via `file:../..`, portanto a etapa raiz `npm run build` é necessária uma vez para gerar `dist/`.

Cada widget/fluxo tem sua própria visualização em `examples/example-showcase/src/views/` que você pode copiar diretamente para o seu próprio aplicativo React.
---