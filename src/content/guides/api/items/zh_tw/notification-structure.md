A `Notification` 物件代表使用者的通知。

`Notification` 物件會自動產生，無法透過 API 建立。它們也會在一年後過期。  
通知無法被刪除。但可以更新將 `viewed` 設為 `false`，且您可以依 `viewed` 進行查詢。

使用者也可以透過將通知中的 `optedOut` 設為 `true` 來取消特定評論的通知。再將其設為 `false` 即可重新訂閱。

有不同的通知類型 ─ 請檢查 `relatedObjectType` 與 `type`。

通知的產生方式相當彈性，可由多種情境觸發（請參考 `NotificationType`）。

截至目前，`Notification` 的存在並不代表會或應該發送電子郵件。相反地，通知用於通知資訊流與相關整合。

`Notification` 物件的結構如下：

[inline-code-attrs-start title = '通知結構'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
enum NotificationObjectType {
    Comment = 0,
    Profile = 1,
    Tenant = 2
}

enum NotificationType {
    /** 如果有人回覆您。 **/
    RepliedToMe = 0,
    /** 如果有人在您曾評論過的討論串的任何位置回覆（即使是子回覆的子回覆）。 **/
    RepliedTransientChild = 1,
    /** 如果您的評論被讚。 **/
    VotedMyComment =2,
    /** 如果在您訂閱的頁面根部留下新評論。 **/
    SubscriptionReplyRoot =3,
    /** 如果有人在您的個人檔案上留言。 **/
    CommentedOnProfile =4,
    /** 如果您收到私訊。 **/
    DirectMessage =5,
    /** TrialLimits 僅適用於租戶使用者。 **/
    TrialLimits =6,
    /** 如果您被 @ 提及。 **/
    Mentioned =7
}

interface Notification {
    id: string
    tenantId: string
    /** 使用 SSO 時，使用者 ID 采用 `<tenant id>:<user id>` 格式。 **/
    userId?: string
    /** 使用 SSO 時，您只需關注 `userId`。 **/
    anonUserId?: string
    /** urlId 幾乎總是已定義。僅在租戶層級的通知（較少見）時才是可選的。 **/
    urlId?: string
    /** URL 已快取，以便快速導向通知來源。 **/
    url?: string
    /** 頁面標題已快取，以便快速閱讀通知來源。 **/
    pageTitle?: string
    relatedObjectType: NotificationObjectType
    /** 例如，評論 ID。 **/
    relatedObjectId: string
    viewed: boolean
    createdAt: string // date string
    type: NotificationType
    fromCommentId?: string
    fromVoteId?: string
    /** fromUserName 與 fromUserAvatarSrc 於此快取，以便快速顯示通知。當使用者資訊更新時，這些也會同步更新。 **/
    fromUserName: string
    fromUserId: string
    fromUserAvatarSrc?: string
    /** 將此設為 true 可停止接收此物件的通知。 **/
    optedOut?: boolean
}
[inline-code-end]