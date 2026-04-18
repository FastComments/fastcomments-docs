Pour voir tous les widgets et flux s'exécutant localement contre le tenant public `demo`, clonez le repo et lancez :

```bash
yarn bootstrap
cd example
yarn ios       # or: yarn android, yarn web
```

Le point d'entrée de la showcase est `example/src/ShowcaseApp.tsx` — une seule application qui expose tous les widgets, thèmes et flux.

La cible `yarn web` utilise `react-native-web` + `react-native-web-webview` (qui rend la WebView sous forme d'iframe). Utile pour des tests rapides d'affichage visuel dans un navigateur ; les API WebView réservées au natif comme `injectJavaScript` et `onShouldStartLoadWithRequest` ne se comporteront pas entièrement sur le web.