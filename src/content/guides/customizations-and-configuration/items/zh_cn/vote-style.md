[related-parameter-start name = 'voteStyle'; type = 'number'; related-parameter-end]

默认情况下，FastComments 会将投票选项呈现为向上和向下的箭头，允许用户对评论进行赞成或反对投票。

但是，可以更改投票工具栏的样式。当前的选项是默认的向上/向下按钮，或者使用心形投票机制。

我们如下使用 **voteStyle** 标志：

[code-example-start config = {voteStyle: 1}; linesToHighlight = [6]; title = '启用心形按钮'; code-example-end]

我们强烈建议您无需编写代码即可完成此操作，因为这还会启用服务器端验证。在小部件自定义页面中，请参阅“投票样式”部分。

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelector = '.vote-style'; selector = '.vote-style'; title='更改投票样式' app-screenshot-end]

也可以禁用投票，请参阅样式选项上方的 `Disable Voting`。

---