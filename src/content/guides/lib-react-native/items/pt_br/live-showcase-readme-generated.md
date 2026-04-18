Para ver todos os widgets e fluxos sendo executados localmente contra o tenant público `demo`, clone o repositório e execute:

```bash
yarn bootstrap
cd example
yarn ios       # or: yarn android, yarn web
```

O ponto de entrada da demonstração é `example/src/ShowcaseApp.tsx` — um único app que expõe todos os widgets, temas e fluxos.

O alvo `yarn web` usa `react-native-web` + `react-native-web-webview` (que renderiza o WebView como um iframe). Útil para testes visuais rápidos em um navegador; APIs de WebView exclusivas do ambiente nativo como `injectJavaScript` e `onShouldStartLoadWithRequest` não se comportarão totalmente na web.