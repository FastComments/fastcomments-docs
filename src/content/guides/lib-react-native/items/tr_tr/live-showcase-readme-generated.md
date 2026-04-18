Halka açık `demo` kiracısına karşı yerel olarak çalışan tüm widget'ları ve akışları görmek için repoyu klonlayın ve çalıştırın:

```bash
yarn bootstrap
cd example
yarn ios       # or: yarn android, yarn web
```

Gösteri giriş noktası `example/src/ShowcaseApp.tsx` — tüm widget'ları, temaları ve akışları açığa çıkaran tek bir uygulamadır.

`yarn web` hedefi `react-native-web` + `react-native-web-webview` kullanır (WebView'i bir iframe olarak render eder). Tarayıcıda hızlı görsel ön testler için kullanışlıdır; `injectJavaScript` ve `onShouldStartLoadWithRequest` gibi yalnızca native WebView API'leri web üzerinde tam olarak davranmayacaktır.