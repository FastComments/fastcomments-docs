`NotificationCount` 对象表示用户的未读通知计数和元数据。

如果没有未读通知，则不会为该用户生成 `NotificationCount`。

`NotificationCount` 对象会自动创建，无法通过 API 创建。它们也会在一年后过期。

您可以通过删除用户的 `NotificationCount` 来清除其未读通知计数。

`NotificationCount` 对象的结构如下：

[inline-code-attrs-start title = 'NotificationCount 结构'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface NotificationCount {
    id: string // 用户 id
    count: number
    createdAt: string // 日期字符串
    expireAt: string // 日期字符串
}
[inline-code-end]