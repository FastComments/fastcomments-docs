A `NotificationCount` 物件表示使用者的未讀通知數量與相關的描述性資訊。

如果沒有未讀通知，該使用者將不會有 `NotificationCount`。

`NotificationCount` 物件會自動建立，無法透過 API 建立。它們也會在一年後過期。

您可以透過刪除使用者的 `NotificationCount` 來清除該使用者的未讀通知數量。

`NotificationCount` 物件的結構如下：

[inline-code-attrs-start title = 'NotificationCount 結構'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface NotificationCount {
    id: string // 使用者 id
    count: number
    createdAt: string // 日期字串
    expireAt: string // 日期字串
}
[inline-code-end]

---