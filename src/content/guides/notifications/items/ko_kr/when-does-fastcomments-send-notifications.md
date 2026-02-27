At FastComments, 우리는 이미 충분한 알림을 받고 있다는 것을 알고 있습니다. 그래서 사용자가 커뮤니티와 계속 연결된 상태를 유지하면서도 받는 알림을 제한하기 위한 몇 가지 조치를 취합니다. 또한 관리자와 중재자에게 최신 상태를 유지시키고 조치가 필요할 때 이를 알리고자 합니다.

#### We'll send notifications for the following events for administrators and moderators:

- Community Digest Summary (frequency configurable).
- Community help requests and reminders.
- New Comments.

#### For Commenters:

- When someone replies to your comment (via email).
- When you are mentioned (in-app and email notification).
- When someone replies in the same thread (in-app and email notification).
- When someone replies to a child comment in the same thread (in-app and email notification).
- When someone replies to a page you have subscribed to (in-app and email notification, frequency configurable per subscription: every minute, hourly, or daily).
- When a user comments for the first time (But not with SSO).
- When a user leaves a comment in a session that is not verified (But not with SSO).
  - We do not send multiple verification emails in this case. Only the first one, which will verify all activity in the same session.

#### For All Users:

- When a login from a new IP address is detected, a security alert email is sent with the approximate location and IP address. This does not apply to the user's very first login.

#### ...and finally for administrators only:

- When integrations are complete.
- When migrations are complete.
- When imports or exports finish.
- When there are billing issues.
- Trial-end reminders.

Some notifications are batched up to prevent mass-sending of notifications to users. Learn about this in the next section `Notification Types`.