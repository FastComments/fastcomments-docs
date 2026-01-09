Interacting with the `Extension` is simple, as we simply define references to functions we want invoked.

To build off the example earlier, let's say we want to add HTML to the top of each comment:

[inline-code-attrs-start title = 'シンプルな拡張機能 - 続き'; type = 'javascript'; inline-code-attrs-end]
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

Whenever you return HTML like this, it will get merged into the UI via a dom-diffing algorithm.

#### コメントの再レンダリングを手動でトリガーする

We can wait for the initial page load and manually re-render a comment by invoking `reRenderComment`:

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
        }, 2000); // タイムアウトは必須ではありません。例として示しています。
    }
})();
[inline-code-end]

---