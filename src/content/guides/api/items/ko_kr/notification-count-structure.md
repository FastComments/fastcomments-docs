A `NotificationCount` object represents the unread notification count and metadata for a user.

If there are no unread notifications, there will be no `NotificationCount` for the user.

`NotificationCount` objects are created automatically and cannot be created via the API. They also expire after one year.

You can clear a user's unread notification count by deleting their `NotificationCount`.

The structure for the `NotificationCount` object is as follows:

[inline-code-attrs-start title = 'NotificationCount 구조'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface NotificationCount {
    id: string // 사용자 id
    count: number
    createdAt: string // 날짜 문자열
    expireAt: string // 날짜 문자열
}
[inline-code-end]