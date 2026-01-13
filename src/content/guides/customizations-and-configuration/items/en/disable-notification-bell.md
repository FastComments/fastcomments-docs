[related-parameter-start name = 'disableNotificationBell'; type = 'boolean'; related-parameter-end]

By default, FastComments will show a notification bell in the top right of the comment area.

This bell will turn red and show a count of the number of notifications the user has. Some example notifications are:

- User replied to you.
- User replied in a thread you commented in.
- User up-voted your comment.
- User replied to a page you have subscribed to.

The notification bell provides a mechanism to subscribe to an entire page, as well.

However, we can disable the notification bell entirely:

[code-example-start config = {disableNotificationBell: true}; linesToHighlight = [6]; title = 'Disable Notification Bell'; code-example-end]

This can also be done without code. In the widget customization page, see the "Disable Notification Bell" section.

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.disable-notification-bell']; selector = '.disable-notification-bell'; title='Disable Notification Bell' app-screenshot-end]
