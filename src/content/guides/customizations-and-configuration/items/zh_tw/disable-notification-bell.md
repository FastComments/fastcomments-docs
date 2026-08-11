[related-parameter-start name = 'disableNotificationBell'; type = 'boolean'; related-parameter-end]

預設情況下，FastComments 會在評論區的右上角顯示通知鈴。

此鈴會變成紅色，並顯示使用者擁有的通知數量。以下是一些範例通知：

- 使用者回覆了您。
- 使用者在您參與的討論串中回覆。
- 使用者為您的評論點讚。
- 使用者回覆了您已訂閱的頁面。

通知鈴同時也提供訂閱整個頁面的機制。

然而，我們可以完全停用通知鈴：

[code-example-start config = {disableNotificationBell: true}; linesToHighlight = [6]; title = 'Disable Notification Bell'; code-example-end]

這也可以不使用程式碼完成。在小工具自訂頁面，請參閱「Disable Notification Bell」區段。

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.disable-notification-bell']; selector = '.disable-notification-bell'; alt='已勾選「Disable Notification Bell」核取方塊的小工具自訂頁面'; title='停用通知鈴' app-screenshot-end]

---