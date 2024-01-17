For SSO there is the following configuration to consider for notifications:

- Whether the user has opted into notifications.
  - This is done by setting the `optedInNotifications` flag to `true` or `false` in the `SSOUser` object.
  - This can be set via the API.
  - Also, if you pass a value for this flag in the payload, it will automatically be updated when the user loads a comment thread.
- Whether the user has opted into **subscription** notifications.
  - This is done by setting the `optedInSubscriptionNotifications` flag to `true` or `false` in the `SSOUser` object.
  - This can be set via the API.
  - Also, if you pass a value for this flag in the payload, it will automatically be updated when the user loads a comment thread.
- Defining their email.
  - If not present, we can't send email based notifications.
- Whether to disable unsubscribe links in emails.
  - This is done via the `disableUnsubscribeLinks` flag in the `Tenant` object.
  - This can be set via the API.
- Whether to use a custom unsubscribe link.
  - This is done via the `footerUnsubscribeURL` property on the `DomainConfig` object.
  - This can be set via the API.
  - You may also want to consider setting the relevant unsubscribe headers via `emailHeaders` in the same object.
