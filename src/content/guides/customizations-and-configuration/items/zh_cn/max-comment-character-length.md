[related-parameter-start name = 'maxCommentCharacterLength'; type = 'number'; related-parameter-end]

评论输入字段允许输入的最大字符数可以通过 **maxCommentCharacterLength** 参数进行限制。

默认值为 2000。

像图片 URL 之类的内容不计入长度计算。

[code-example-start config = {maxCommentCharacterLength: 500}; linesToHighlight = [6]; title = '限制评论长度'; code-example-end]

这可以在无需编码的情况下在小部件自定义页面上进行自定义：

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.max-comment-size'; title='限制评论长度' app-screenshot-end]

---