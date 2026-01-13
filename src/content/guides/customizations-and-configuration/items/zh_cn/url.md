[related-parameter-start name = 'url'; type = 'string'; related-parameter-end]

发送通知邮件，或在像审核页面这样的用户界面中呈现评论时，能够将链接
从评论指向其所在页面会很有帮助。

如果 URL ID 不总是一个 ID，那么我们必须在别处存储 URL。这就是 "url" 属性的用途，定义如下。

[code-example-start config = {url: 'https://example.com/article-5'}; linesToHighlight = [6]; title = 'Defining a Custom URL'; code-example-end]

一个常见用例是将评论线程绑定到一个标识符（例如文章），然后链接回特定页面，例如：

[code-example-start config = {url: 'https://example.com/article-5', urlId: 'article-5'}; linesToHighlight = [6, 7]; title = 'Defining Custom URL and URL IDs together'; code-example-end]

URL 不会去除常见的营销参数。默认情况下，当前页面的 URL 即为随评论存储的 URL。