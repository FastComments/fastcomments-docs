---
[related-parameter-start name = 'disableSuccessMessage'; type = 'boolean'; related-parameter-end]

默认情况下，FastComments 在评论后会显示成功消息。可以按以下方式禁用它：

[code-example-start config = {disableSuccessMessage: true}; linesToHighlight = [6]; title = 'Disable Success Message'; code-example-end]

这也可以在不使用代码的情况下完成。在小部件自定义页面：

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.disable-success-message']; selector = '.disable-success-message'; alt='小部件自定义页面，已勾选“禁用成功消息”复选框以隐藏评论后的确认信息'; title='禁用成功消息' app-screenshot-end]

---