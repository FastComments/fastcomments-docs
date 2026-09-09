---
Aby zobaczyć każdy widget i flow działające lokalnie przeciwko publicznemu tenantowi `demo`, sklonuj repozytorium i uruchom:

```bash
npm install
npm run build
cd examples/example-showcase
npm install
npm run dev
```

Przykłady odwołują się do biblioteki poprzez `file:../..`, więc krok `npm run build` w katalogu głównym jest potrzebny raz, aby wygenerować `dist/`.

Każdy widget/flow ma własny widok w `examples/example-showcase/src/views/`, który możesz skopiować bezpośrednio do swojej aplikacji React.
---