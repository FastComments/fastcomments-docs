---
[related-parameter-start name = 'disableNotificationBell'; type = 'boolean'; related-parameter-end]

默认情况下，FastComments 会在评论区域的右上角显示通知铃铛。

该铃铛会变为红色并显示用户收到的通知数量。以下是一些示例通知：

- 用户回复了你。
- 用户在你评论过的线程中回复了。
- 用户为你的评论点赞。
- 用户回复了你订阅的页面。

通知铃铛还提供了订阅整个页面的机制。

然而，我们可以完全禁用通知铃铛：

[code-example-start config = {disableNotificationBell: true}; linesToHighlight = [6]; title = 'Disable Notification Bell'; code-example-end]

这也可以在不使用代码的情况下完成。在小部件自定义页面中，请参阅“Disable Notification Bell”部分。

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.disable-notification-bell']; selector = '.disable-notification-bell'; title='Disable Notification Bell' app-screenshot-end]

---