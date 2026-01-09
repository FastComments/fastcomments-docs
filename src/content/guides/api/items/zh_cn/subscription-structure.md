A `Subscription` object 表示用户的订阅。

`Subscription` objects 在用户在评论小部件中点击通知铃铛并点击 "订阅此页面" 时创建。

Subscriptions 也可以通过 API 创建。

Having a `Subscription` object causes `Notification` objects to be generated, and emails sent, when new comments are left on the root of the associated page
that the `Subscription` is for. 电子邮件的发送取决于用户类型。对于普通用户，这取决于 `optedInNotifications`。对于 SSO 用户，这取决于 `optedInSubscriptionNotifications`。注意，有些应用可能没有可通过 Web 访问的页面的概念，在这种情况下，只需将 `urlId` 设置为您要订阅的项目的 id（与传递给评论小部件的 `urlId` 值相同）。

The structure for the `Subscription` object is as follows:

[inline-code-attrs-start title = '订阅结构'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface Subscription {
    id: string
    tenantId: string
    /** 使用 SSO 时，用户 ID 的格式为 `<tenant id>:<user id>`。 **/
    userId: string
    anonUserId?: string
    urlId: string
    url?: string
    pageTitle?: string
    createdAt: string // 日期字符串
}
[inline-code-end]

---