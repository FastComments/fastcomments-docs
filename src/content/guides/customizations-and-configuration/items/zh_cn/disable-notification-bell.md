[related-parameter-start name = 'disableNotificationBell'; type = 'boolean'; related-parameter-end]

默认情况下，FastComments 会在评论区域的右上角显示一个通知铃。

该铃会变红并显示用户拥有的通知数量。一些示例通知包括：

- 用户回复了您。
- 用户在您参与的线程中回复。
- 用户给您的评论点了赞。
- 用户回复了您订阅的页面。

通知铃还提供了订阅整个页面的机制。

但是，我们可以完全禁用通知铃：

[code-example-start config = {disableNotificationBell: true}; linesToHighlight = [6]; title = '禁用通知铃'; code-example-end]

这也可以在不写代码的情况下完成。在小部件自定义页面，查看“禁用通知铃”部分。

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.disable-notification-bell']; selector = '.disable-notification-bell'; alt='已勾选禁用通知铃复选框的小部件自定义页面'; title='禁用通知铃' app-screenshot-end]