[related-parameter-start name = 'voteStyle'; type = 'number'; related-parameter-end]

默认情况下，FastComments 会将投票选项渲染为向上和向下的箭头，允许用户对评论进行赞成或反对投票。

但是，可以更改投票工具栏的样式。当前的选项是默认的上下按钮，或使用心形投票机制。

我们使用 **voteStyle** 标志如下：

[code-example-start config = {voteStyle: 1}; linesToHighlight = [6]; title = '启用心形按钮'; code-example-end]

我们强烈建议您在不使用代码的情况下完成此操作，因为它还会启用服务器端验证。在小部件自定义页面，参见 “Vote Style” 部分。

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelector = '.vote-style'; selector = '.vote-style'; alt='在小部件自定义页面上的投票样式设置，提供上下箭头或心形投票'; title='更改投票样式' app-screenshot-end]

投票也可以被禁用，请参见样式选项上方的 `Disable Voting`。