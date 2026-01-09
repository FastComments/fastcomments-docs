作成できる最小の拡張機能は次のとおりです：

[inline-code-attrs-start title = 'シンプルな拡張'; type = 'javascript'; inline-code-attrs-end]
[inline-code-start]
(function () {
    const extension = FastCommentsUI.extensions.find((extension) => {
        return extension.id === 'my-extension';
    });
})();
[inline-code-end]

この例のために、これを `my-extension.js` として保存し、`https://example.com/my-extension.min.js` で公開してください。

この拡張機能は何もしません。読み込み時にコアのコメントライブラリによって作成された拡張オブジェクトを取得するだけです。

この `Extension` オブジェクトはシングルトンで、他の拡張機能とは共有されません。

次に、拡張機能を読み込むには、コメントウィジェットにそのことを伝える必要があります。例えば：

[code-example-start config = {extensions: [{id: "my-extension", path: "https://example.com/my-extension.min.js"}]}; linesToHighlight = [6,7,8,9,10,11]; title = 'Using a Custom Extension'; code-example-end]

機能的な例については、次のセクションを参照してください。

---