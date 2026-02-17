---
У FastComments ми знаємо, що ви й так отримуєте забагато сповіщень. Через це ми вдаємося до певних заходів, щоб обмежити кількість сповіщень, які отримують користувачі, при цьому
тримуючи їх на зв'язку зі своїми спільнотами. Ми також хочемо тримати адміністраторів та модераторів в курсі подій і повідомляти їм, коли потрібно вжити заходів.

#### Ми надсилатимемо сповіщення про такі події для адміністраторів і модераторів:

- Community Digest Summary (frequency configurable).
- Community help requests and reminders.
- New Comments.

#### Для коментаторів:

- When someone replies to your comment (via email).
- When you are mentioned (in-app and email notification).
- When someone replies in the same thread (in-app and email notification).
- When someone replies to a child comment in the same thread (in-app and email notification).
- When someone replies to a page you have subscribed to (in-app and email notification, frequency configurable per subscription: every minute, hourly, or daily).
- When a user comments for the first time (But not with SSO).
- When a user leaves a comment in a session that is not verified (But not with SSO).
  - We do not send multiple verification emails in this case. Only the first one, which will verify all activity in the same session.

#### ...і нарешті тільки для адміністраторів:

- When integrations are complete.
- When migrations are complete.
- When imports or exports finish.
- When there are billing issues.
- Trial-end reminders.

Деякі сповіщення групуються, щоб запобігти масовій розсилці користувачам. Learn about this in the next section `Notification Types`.

---