### ダークモード対応

### 動的ダークモード

サイトのダークモードが body 要素に `.dark` クラスを追加して制御されている場合、Collab Chat UI は再初期化を必要とせずにこれを自動的に反映します。ウィジェットのスタイルは、このクラスの有無に応じて応答するように設計されています。

[inline-code-attrs-start title = 'ダークモードのCSS例'; type = 'css'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
/* ダークモード用のCSS */
body.dark {
    background: #1a1a1a;
    color: #ffffff;
}
[inline-code-end]

### CSSによるカスタムスタイル

ハイライト、チャットウィンドウ、その他の要素の外観は CSS でカスタマイズできます。ウィジェットはスタイルシートでターゲットにできる特定のクラスを追加します。

テキストハイライトは FastComments のコメントバブルスタイリングシステムを使用しているため、標準のコメントウィジェットに適用したカスタマイズは Collab Chat にも影響します。

### トップバーのカスタマイズ

トップバーにはオンラインのユーザー数とディスカッション数が表示されます。`topBarTarget` としてカスタム要素を指定することで位置をカスタマイズできます:

[inline-code-attrs-start title = 'カスタムトップバーの位置'; type = 'javascript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
FastCommentsCollabChat(element, {
    tenantId: 'demo',
    topBarTarget: document.getElementById('my-custom-header')
});
[inline-code-end]

または `null` に設定して完全に無効化することもできます:

[inline-code-attrs-start title = 'トップバーを無効化'; type = 'javascript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
FastCommentsCollabChat(element, {
    tenantId: 'demo',
    topBarTarget: null
});
[inline-code-end]

### モバイルでの挙動

幅が 768px 未満の画面では、Collab Chat は自動的にモバイル最適化レイアウトに切り替わります。チャットウィンドウはテキストの横に浮かぶ代わりにフルスクリーンで表示され、選択の遅延がなくなり、より即時の操作が可能になります。

この挙動は組み込みで、設定は不要です。ウィジェットは画面サイズを自動的に検出して調整します。

### チャットウィンドウの外観

チャットウィンドウはデスクトップで幅が 410px、ハイライトされたテキストを指す 16px の矢印があります。ウィンドウは利用可能なビューポート空間に基づいて自動的に位置決めされ、`to-right`、`to-left`、`to-top`、`to-bottom` のようなポジショニングクラスを使用します。

これらのウィンドウの色、フォント、間隔、その他の視覚的プロパティを調整するためにカスタム CSS を追加できます。チャットウィンドウは標準の FastComments ウィジェットと同じコンポーネント構造を使用しているため、適用したグローバルなカスタマイズを継承します。

### ローカリゼーション

Collab Chat は標準の FastComments ウィジェットと同じローカリゼーションオプションをすべてサポートします。UI テキストを別の言語で表示するには `locale` オプションを設定してください:

[inline-code-attrs-start title = 'ロケールを設定'; type = 'javascript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
FastCommentsCollabChat(element, {
    tenantId: 'demo',
    locale: 'es'  // スペイン語
});
[inline-code-end]

FastComments は数十の言語をサポートしています。ロケール設定はプロンプト、ボタン、プレースホルダーテキストなど、すべての UI テキストに影響します。

### 継承されるカスタマイズオプション

Collab Chat は標準のコメントウィジェットを拡張しているため、ベースウィジェットからすべてのカスタマイズオプションを継承します。これにはカスタム CSS クラス、カスタム翻訳、アバターのカスタマイズ、日付のフォーマットなど、多くの項目が含まれます。

利用可能なカスタマイズオプションの完全な一覧については、メインの FastComments カスタマイズドキュメントを参照してください。

### カスタムフォントの利用

サイトがカスタムフォントを使用している場合、Collab Chat UI はページの CSS からそれらのフォントを継承します。フローティングチャットウィンドウにも同じフォントを使用させたい場合は、ウィジェットカスタマイズルールを作成し、そのルール内のカスタム CSS で `@import` によってフォントを読み込む必要があるかもしれません。