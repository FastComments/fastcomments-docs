最小可行的扩展如下：

[inline-code-attrs-start title = '一个简单的扩展'; type = 'javascript'; inline-code-attrs-end]
[inline-code-start]
(function () {
    const extension = FastCommentsUI.extensions.find((extension) => {
        return extension.id === 'my-extension';
    });
})();
[inline-code-end]

为了这个示例，将其保存为 `my-extension.js`，并使其可以通过 `https://example.com/my-extension.min.js` 访问。

此扩展不会执行任何操作，除了在加载时它会获取由核心评论库创建的扩展对象。

该 `Extension` 对象是单例的，不会与其他扩展共享。

接下来，要加载我们的扩展，必须告知评论小部件有关它。例如：

[code-example-start config = {extensions: [{id: "my-extension", path: "https://example.com/my-extension.min.js"}]}; linesToHighlight = [6,7,8,9,10,11]; title = 'Using a Custom Extension'; code-example-end]

有关功能示例，请参见下一节。

---