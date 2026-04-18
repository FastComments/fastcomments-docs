パブリックな `demo` テナントに対してローカルで実行されているすべてのウィジェットとフローを確認するには、リポジトリをクローンして次を実行してください：

```bash
yarn bootstrap
cd example
yarn ios       # or: yarn android, yarn web
```

ショーケースのエントリーポイントは `example/src/ShowcaseApp.tsx` です — すべてのウィジェット、テーマ、およびフローを公開する単一のアプリです。

`yarn web` ターゲットは `react-native-web` + `react-native-web-webview` を使用します（WebView を iframe としてレンダリングします）。ブラウザでの簡易的な視覚スモークテストに便利ですが、`injectJavaScript` や `onShouldStartLoadWithRequest` のようなネイティブ専用の WebView API はウェブ上では完全には機能しません。