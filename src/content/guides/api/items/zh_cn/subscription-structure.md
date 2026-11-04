`Subscription` 对象表示用户的订阅。

当用户在评论组件中点击通知铃铛并点击“订阅此页面”时，会创建 `Subscription` 对象。

订阅也可以通过 API 创建。

拥有 `Subscription` 对象后，当在 `Subscription` 所关联页面的根目录下有新评论时，会生成 `Notification` 对象并发送电子邮件。是否发送电子邮件取决于用户类型。对于普通用户，这取决于 `optedInNotifications`。对于 SSO 用户，这取决于 `optedInSubscriptionNotifications`。请注意，某些应用程序可能没有网页可访问页面的概念，在这种情况下，只需将 `urlId` 设置为您订阅的项目的 id（与传递给评论组件的 `urlId` 值相同）。

`Subscription` 对象的结构如下：

[inline-code-attrs-start title = 'Subscription 结构'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface Subscription {
    id: string
    tenantId: string
    /** 使用 SSO 时，用户 id 的格式为 `<tenant id>:<user id>`。 **/
    userId: string
    anonUserId?: string
    urlId: string
    url?: string
    pageTitle?: string
    createdAt: string // 日期字符串
}
[inline-code-end]