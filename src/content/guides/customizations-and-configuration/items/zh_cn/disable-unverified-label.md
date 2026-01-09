[related-parameter-start name = 'disableUnverifiedLabel'; type = 'boolean'; related-parameter-end]

默认情况下，FastComments 会为那些
具有未验证浏览器会话的用户留下的评论显示“未验证的评论”标签。有关未验证评论的更多信息，请在[此处](https://docs.fastcomments.com/guide-comment-vote-verification.html)查看。

[code-example-start config = {disableUnverifiedLabel: true}; linesToHighlight = [6]; title = 'Disable The Unverified Label'; code-example-end]

此外，此功能也可以在自定义 UI 中使用，无需编写代码：

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.disable-unverified-comment-label']; selector = '.disable-unverified-comment-label'; title='Disable The Unverified Label' app-screenshot-end]

---