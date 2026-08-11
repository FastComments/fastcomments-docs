[related-parameter-start name = 'commentCountFormat'; type = 'string'; related-parameter-end]

在评论小部件顶部显示的评论计数可以自定义。

这可以替换为任意字符串，且值 **[count]** 将被替换为计数值，并为用户本地化。

[code-example-start config = {commentCountFormat: "There are [count] comments."}; linesToHighlight = [6]; title = '自定义评论计数文本'; code-example-end]

这可以在无需代码的情况下，通过小部件自定义页面进行自定义：

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.comment-count'; alt='小部件自定义页面上的评论计数字段，其中 [count] 将被替换为实时总数'; title='自定义评论计数文本' app-screenshot-end]