Da biste vidjeli svaki widget i flow koji se izvršava lokalno u javnom `demo` tenantu, klonirajte repozitorij i pokrenite:

```bash
yarn bootstrap
cd example
yarn ios       # or: yarn android, yarn web
```

Ulazna točka pokazne aplikacije je `example/src/ShowcaseApp.tsx` — jedna aplikacija koja prikazuje sve widgete, teme i tokove.

Cilj `yarn web` koristi `react-native-web` + `react-native-web-webview` (koji prikazuje WebView kao iframe). Korisno za brze vizualne smoke testove u pregledniku; WebView API-ji koji su samo za native, poput `injectJavaScript` i `onShouldStartLoadWithRequest`, neće se u potpunosti ponašati na webu.