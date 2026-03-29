このライブラリは [FastComments](https://fastcomments.com) の完全な react-native 実装です。

ライブコメント、チャット、スレッド、絵文字、通知、SSO、スキンをサポートしており、スタイルシートオブジェクトを渡すことで完全にカスタマイズできます。すべてのアセットもカスタマイズ可能で、ダークモードに応じて異なるアセットを切り替えることもサポートしています。

このライブラリの利点は、`fastcomments-react-native` ラッパーよりも柔軟であることです。コメントは webview 内ではなくネイティブコンポーネントでレンダリングされます。注意: リッチテキストエディタの推移的な依存関係として `react-native-webview` がまだ必要です（`@10play/tentap-editor`）。

すべて FastComments のバックエンド上で動作するため、UI を組み込むだけで済みます:

```tsx
    <FastCommentsLiveCommenting config={config} styles={styles} callbacks={callbacks} assets={assets}/>
```

詳しい例は [example/src](https://github.com/FastComments/fastcomments-react-native-sdk/blob/main/example/src) を参照してください。

既存の React Native アプリにライブチャットを追加する、あるいはソーシャルネットワークを構築することもできます！