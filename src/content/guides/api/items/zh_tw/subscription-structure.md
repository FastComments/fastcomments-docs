一個 `Subscription` 物件代表使用者的訂閱。

`Subscription` 物件會在使用者在評論小工具中點擊通知鈴鐺並點選 "訂閱此頁面" 時建立。

也可以透過 API 建立訂閱。

擁有 `Subscription` 物件會在與該 `Subscription` 相關聯的頁面
根層留下新留言時，會產生 `Notification` 物件並發送電子郵件。

是否發送電子郵件取決於使用者類型。對於一般使用者，這取決於 `optedInNotifications`。對於 SSO 使用者，則取決於 `optedInSubscriptionNotifications`。請注意，有些應用程式可能沒有可由網頁存取的頁面概念，在這種情況下，只需將 `urlId` 設為
您要訂閱的項目的 id（與傳遞給評論小工具的 `urlId` 值相同）。

以下為 `Subscription` 物件的結構：

[inline-code-attrs-start title = '訂閱結構'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface Subscription {
    id: string
    tenantId: string
    /** 使用 SSO 時，使用者 id 的格式為 `<tenant id>:<user id>`。 **/
    userId: string
    anonUserId?: string
    urlId: string
    url?: string
    pageTitle?: string
    createdAt: string // 日期字串
}
[inline-code-end]

---