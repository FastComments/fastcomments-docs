Para ver cada widget y flujo ejecutándose localmente contra el inquilino público `demo`, clona el repositorio y ejecuta:

```bash
npm install
npm run build
cd examples/example-showcase
npm install
npm run dev
```

Los ejemplos enlazan a la biblioteca mediante `file:../..`, por lo que el paso raíz `npm run build` es necesario una vez para generar `dist/`.

Cada widget/flujo tiene su propia vista bajo `examples/example-showcase/src/views/` que puedes copiar directamente en tu propia aplicación React.