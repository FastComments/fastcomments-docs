[related-parameter-start name = 'useSingleLineCommentInput'; type = 'boolean'; related-parameter-end]

默认情况下，FastComments 会允许用户输入任意行数的评论，直至默认字符限制。

然而，可能需要限制用户只能输入单行文本。示例用例包括在线竞标或实时聊天，FastComments 可用于这些场景。

我们按如下方式启用 **useSingleLineCommentInput** 标志：

[code-example-start config = {useSingleLineCommentInput: true}; linesToHighlight = [6]; title = 'Enable Single-Line Comment Input'; code-example-end]

这也可以不通过代码实现。在小部件自定义页面，查看 “Enable Single-Line Comment Input” 部分。

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelector = '.single-line-comment-input'; selector = '.single-line-comment-input'; alt='在小部件自定义页面中打开的单行评论输入复选框，限制输入为单行'; title='启用单行评论输入' app-screenshot-end]

请注意，每个页面的每个排序方向的评论都是预先计算的，因此所有排序方向的性能相同。