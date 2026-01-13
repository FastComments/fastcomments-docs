[related-parameter-start name = 'enableInfiniteScrolling'; type = 'boolean'; related-parameter-end]

デフォルトでは、FastComments ウィジェットは表示されているすべてのコメントに合わせて縦方向に自動的にサイズを調整します。ページネーションは現在のページ末尾にある「次を表示」ボタンで行われます。これは多くのユーザーにとって最も自然に感じられる操作方法であると判断しているためです。

しかし、無限スクロールの方が好まれるケースもあります。例えば、Stream Chat 製品ではこの機能を使用しています。

**enableInfiniteScrolling** フラグを true に設定することで、「次を表示」ボタンを非表示にして無限スクロールに切り替えることができます:

[code-example-start config = {enableInfiniteScrolling: true}; linesToHighlight = [6]; title = 'Enabling Infinite Scrolling'; code-example-end]

これにはカスタム CSS の追加も必要です。スクロールを有効にするには、`.comments` セレクタに対して次のようなカスタム CSS を追加します。例えば：

[inline-code-attrs-start title = '無限スクロールを有効にする'; type = 'css'; inline-code-attrs-end]
[inline-code-start]
.comments {
    max-height: 500px;
    overflow-y: auto;
}
[inline-code-end]

完全な動作例は次のとおりです：

[code-example-start config = {enableInfiniteScrolling: true, customCSS: '.comments { max-height: 500px;  overflow-y: auto; }'}; linesToHighlight = [6, 7]; title = 'Enabling Infinite Scrolling'; code-example-end]

上の例では `customCSS` プロパティを使用していますが、パフォーマンスの観点からは代わりにウィジェット設定 UI を使用することを推奨します。 [カスタム CSS のドキュメントを参照してください。](/guide-customizations-and-configuration.html#custom-css)