與 `Extension` 的互動很簡單，只要定義我們想呼叫的函式參考。

延續先前的範例，假設我們想在每個評論的頂部加入 HTML：

[inline-code-attrs-start title = '簡單的擴充功能 - 繼續'; type = 'javascript'; inline-code-attrs-end]
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

每當你返回這類 HTML 時，它會透過 DOM 差異比對演算法合併到 UI 中。

#### 手動觸發評論的重新渲染

我們可以等到初始頁面載入完成，並透過呼叫 `reRenderComment` 手動重新渲染評論：

[inline-code-attrs-start title = '重新渲染評論'; type = 'javascript'; inline-code-attrs-end]
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

---