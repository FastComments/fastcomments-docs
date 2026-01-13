このライブラリは、[FastComments](https://fastcomments.com)の完全な react-native 実装です。

ライブコメント、チャット、スレッド、絵文字、通知、SSO、スキンをサポートしており、stylesheet オブジェクトを渡すことで完全にカスタマイズできます。すべての assets もカスタマイズ可能で、ダークモードに応じて異なる assets を切り替えることもサポートしています。

このライブラリの利点は、より柔軟であり、`fastcomments-react-native` ラッパーのように webview を必要としないことです。

すべては FastComments のバックエンド上で動作するため、UI を組み込むだけで済みます:

```tsx
    <FastCommentsLiveCommenting config={config} styles={styles} callbacks={callbacks} assets={assets}/>
```

詳細な例は [example/src](https://github.com/FastComments/fastcomments-react-native-sdk/blob/main/example/src) を参照してください。

既存の React Native アプリケーションにライブチャットを追加したり、ソーシャルネットワークを構築したりできます！