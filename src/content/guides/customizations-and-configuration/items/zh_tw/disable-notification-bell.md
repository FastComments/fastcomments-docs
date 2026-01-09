[related-parameter-start name = 'disableNotificationBell'; type = 'boolean'; related-parameter-end]

預設情況下，FastComments 會在評論區的右上方顯示一個通知鈴。

這個鈴會變成紅色並顯示使用者的通知數量。有些通知範例包括：

- 使用者回覆了您。
- 使用者回覆了您曾留言的討論串。
- 使用者對您的評論按了讚。
- 使用者回覆了您已訂閱的頁面。

通知鈴也提供訂閱整個頁面的機制。

不過，我們可以完全停用通知鈴：

[code-example-start config = {disableNotificationBell: true}; linesToHighlight = [6]; title = 'Disable Notification Bell'; code-example-end]

這也可以在不撰寫程式碼的情況下完成。在小工具自訂頁面，請參閱「停用通知鈴」區段。

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.disable-notification-bell']; selector = '.disable-notification-bell'; title='Disable Notification Bell' app-screenshot-end]