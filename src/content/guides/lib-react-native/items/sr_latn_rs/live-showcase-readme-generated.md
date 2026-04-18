Da biste videli sve widgete i tokove koji rade lokalno protiv javnog `demo` tenant-a, klonirajte repo i pokrenite:

```bash
yarn bootstrap
cd example
yarn ios       # or: yarn android, yarn web
```

Ulazna tačka showcase-a je `example/src/ShowcaseApp.tsx` — jedna aplikacija koja prikazuje sve widgete, teme i tokove.

Cilj `yarn web` koristi `react-native-web` + `react-native-web-webview` (koji renderuje WebView kao iframe). Korisno za brze vizuelne smoke testove u pregledaču; nativni WebView API-ji koji su samo za native, kao što su `injectJavaScript` i `onShouldStartLoadWithRequest`, neće se u potpunosti ponašati na webu.