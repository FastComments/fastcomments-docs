`Extension` との連携はシンプルです。実行したい関数への参照を定義するだけだからです。

先ほどの例を踏まえて、各コメントの先頭にHTMLを追加したい場合を考えてみましょう:

[inline-code-attrs-start title = 'シンプルな Extension - 続き'; type = 'javascript'; inline-code-attrs-end]
[inline-code-start]
(function () {
    const extension = FastCommentsUI.extensions.find((extension) => {
        return extension.id === 'my-extension';
    });

    extension.commentFilter = function(comment, html) {
        return `<h3>The user's name is ${comment.commenterName}!</h3>` + html;
    }
})();
[inline-code-end]

このようにHTMLを返すと、dom-diffingアルゴリズムを通じてUIにマージされます。

#### コメントの再レンダリングを手動でトリガーする

初回のページ読み込みを待った後、`reRenderComment` を呼び出すことでコメントを手動で再レンダリングできます:

[inline-code-attrs-start title = 'コメントの再レンダリング'; type = 'javascript'; inline-code-attrs-end]
[inline-code-start]
(function () {
    const extension = FastCommentsUI.extensions.find((extension) => {
        return extension.id === 'my-extension';
    });

    let renderCount = 0;

    extension.commentFilter = function(comment, html) {
        renderCount++;
        return `<h3>The render count is ${renderCount}!</h3>` + html;
    }

    extension.onInitialRenderComplete = function() {
        setInterval(function() {
            extension.reRenderComment(extension.commentsById[Object.keys(extension.commentsById)[0]], function renderDone() {
                console.log('Comment re-render done.');
            });
        }, 2000); // timeout not required, just an example.
    }
})();
[inline-code-end]