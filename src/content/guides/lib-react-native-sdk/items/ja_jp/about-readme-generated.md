---
このライブラリは [FastComments](https://fastcomments.com) の完全な react-native 実装です。

ライブコメント、チャット、スレッド、絵文字、通知、SSO、スキン、およびスタイルシートオブジェクトを渡すことでの完全なカスタマイズをサポートします。すべてのアセットもカスタマイズ可能で、ダークモードに応じて異なるアセットを切り替えることができます。

このライブラリの利点は、`fastcomments-react-native` ラッパーよりも柔軟であることです。コメントは webview の中ではなくネイティブコンポーネントでレンダリングされます。

すべて FastComments のバックエンド上で動作するため、UI を組み込むだけで済みます：

```tsx
    <FastCommentsLiveCommenting config={config} styles={styles} callbacks={callbacks} assets={assets}/>
```

さらに詳しい例は [example/src](https://github.com/FastComments/fastcomments-react-native-sdk/blob/main/example/src) を参照してください。

既存の React Native アプリケーションにライブチャットを追加するか、あるいはソーシャルネットワークを構築することもできます！
---