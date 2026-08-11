[related-parameter-start name = 'maxCommentCharacterLength'; type = 'number'; related-parameter-end]

评论输入字段允许输入的最大字符数可以通过 **maxCommentCharacterLength** 参数进行限制。

默认值为 2000。

类似图片 URL 的内容不计入字符长度的计算。

[code-example-start config = {maxCommentCharacterLength: 500}; linesToHighlight = [6]; title = '限制评论长度'; code-example-end]

这可以在小部件自定义页面上无需代码进行设置：

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.max-comment-size'; alt='小部件自定义页面上的最大评论大小字段，用于限制评论可包含的字符数'; title='限制评论长度' app-screenshot-end]

---