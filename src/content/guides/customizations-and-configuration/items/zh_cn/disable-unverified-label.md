[related-parameter-start name = 'disableUnverifiedLabel'; type = 'boolean'; related-parameter-end]

默认情况下，FastComments 会为留下的评论显示“未验证评论”标签，针对已留下评论的用户
其浏览器会话未验证。了解更多关于未验证评论的内容，请点击[此处](https://docs.fastcomments.com/guide-comment-vote-verification.html)。

[code-example-start config = {disableUnverifiedLabel: true}; linesToHighlight = [6]; title = '禁用未验证标签'; code-example-end]

此外，您可以在自定义 UI 中使用此功能，而无需编写代码：

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.disable-unverified-comment-label']; selector = '.disable-unverified-comment-label'; alt='已勾选“禁用未验证评论标签”复选框的部件自定义页面'; title='禁用未验证标签' app-screenshot-end]

---