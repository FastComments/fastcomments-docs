对于 SSO，关于通知需要考虑以下配置：

- 用户是否选择接收通知。
  - 通过在 `SSOUser` 对象中将 `optedInNotifications` 标志设置为 `true` 或 `false` 来完成。
  - 这可以通过 API 设置。
  - 此外，如果在有效负载中为此标志传递了一个值，当用户加载评论线程时该值将自动更新。
- 用户是否选择接收 **subscription** 通知。
  - 通过在 `SSOUser` 对象中将 `optedInSubscriptionNotifications` 标志设置为 `true` 或 `false` 来完成。
  - 这可以通过 API 设置。
  - 此外，如果在有效负载中为此标志传递了一个值，当用户加载评论线程时该值将自动更新。
- 定义他们的电子邮件。
  - 如果未提供，我们无法发送基于电子邮件的通知。
- 是否在电子邮件中禁用退订链接。
  - 通过 `Tenant` 对象中的 `disableUnsubscribeLinks` 标志来完成。
  - 这可以通过 API 设置。
- 是否使用自定义退订链接。
  - 通过 `DomainConfig` 对象上的 `footerUnsubscribeURL` 属性来完成。
  - 这可以通过 API 设置。
  - 您也可以考虑在同一对象中通过 `emailHeaders` 设置相关的退订头。