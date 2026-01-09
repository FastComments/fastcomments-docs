[related-parameter-start name = 'useSingleLineCommentInput'; type = 'boolean'; related-parameter-end]

默认情况下，FastComments 允许用户输入任意行数的评论，直到达到默认的字符限制。

然而，可能希望将用户限制为只输入单行文本。一些示例用例包括在线竞价，或实时聊天，FastComments
可用于这些场景。

我们通过以下方式启用 **useSingleLineCommentInput** 标志：

[code-example-start config = {useSingleLineCommentInput: true}; linesToHighlight = [6]; title = 'Enable Single-Line Comment Input'; code-example-end]

也可以无需编写代码来实现。在小部件自定义页面中，参见“启用单行评论输入”部分。

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelector = '.single-line-comment-input'; selector = '.single-line-comment-input'; title='Enable Single-Line Comment Input' app-screenshot-end]

注意，每个页面上针对每种排序方向的评论都是预先计算的，因此所有排序方向具有相同的性能。

---