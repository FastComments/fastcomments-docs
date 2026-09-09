---
Pour voir chaque widget et flux s'exécutant localement contre le locataire public `demo`, clonez le dépôt et exécutez :

```bash
npm install
npm run build
cd examples/example-showcase
npm install
npm run dev
```

Les exemples se lient à la bibliothèque via `file:../..`, donc l'étape racine `npm run build` est nécessaire une fois pour produire `dist/`.

Chaque widget/flux possède sa propre vue sous `examples/example-showcase/src/views/` que vous pouvez copier directement dans votre propre application React.
---