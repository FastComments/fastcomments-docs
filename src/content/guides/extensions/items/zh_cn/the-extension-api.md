与 `Extension` 交互很简单，因为我们只需定义要调用的函数的引用。

基于前面的示例，假设我们想在每条评论的顶部添加 HTML：

[inline-code-attrs-start title = '一个简单的扩展 - 续篇'; type = 'javascript'; inline-code-attrs-end]
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

每当你以这种方式返回 HTML 时，它会通过一个 DOM 差分算法合并到 UI 中。

#### 手动触发评论的重新渲染

我们可以等待初始页面加载完成，然后通过调用 `reRenderComment` 手动重新渲染某条评论：

[inline-code-attrs-start title = '重新渲染评论'; type = 'javascript'; inline-code-attrs-end]
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