FastComments 提供易於使用的 SSO 解決方案。使用基於 HMAC 的整合更新使用者資訊，就像讓使用者載入帶有更新載荷的頁面一樣簡單。

不過，您可能會希望在該流程之外管理使用者，以改善應用程式的一致性。

SSO 使用者 API 提供一種對我們稱為 SSOUser 的物件進行 CRUD 的方式。這些物件不同於一般的 Users，並為了型別安全而分開保存。

SSOUser 物件的結構如下：

[inline-code-attrs-start title = 'SSOUser 結構'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface SSOUser {
    id: string
    username: string
    email?: string
    websiteUrl?: string
    signUpDate: number
    createdFromUrlId?: string
    loginCount?: number
    avatarSrc?: string
    optedInNotifications?: boolean
    optedInSubscriptionNotifications?: boolean
    displayLabel?: string
    displayName?: string
    isAccountOwner?: boolean // 管理員權限 - 有此標記的 SSO 使用者會以 SSO 管理員身分計費（與一般 SSO 使用者分開）
    isAdminAdmin?: boolean // 管理員權限 - 有此標記的 SSO 使用者會以 SSO 管理員身分計費（與一般 SSO 使用者分開）
    isCommentModeratorAdmin?: boolean // 版主權限 - 有此標記的 SSO 使用者會以 SSO 版主身分計費（與一般 SSO 使用者分開）
    /** 如果為 null，則不會將存取控制套用到該使用者。如果為空列表，該使用者將無法看到任何頁面或在 @ 提及其他使用者。 **/
    groupIds?: string[] | null
    createdFromSimpleSSO?: boolean
    /** 不要讓其他使用者在其個人檔案上看到此使用者的活動（包括留言）。預設為 true，以預設提供更安全的個人檔案。 **/
    isProfileActivityPrivate?: boolean
    /** 不要讓其他使用者在該使用者的個人檔案上留言，或看到現有的個人檔案留言。預設為 false。 **/
    isProfileCommentsPrivate?: boolean
    /** 不要讓其他使用者傳送直接訊息給此使用者。預設為 false。 **/
    isProfileDMDisabled?: boolean
    karma?: number
    /** 使用者徽章的可選設定。 **/
    badgeConfig?: {
        /** 分配給使用者的徽章 ID 陣列。最多限制為 30 個徽章。順序會被保留。 **/
        badgeIds: string[]
        /** 如果為 true，將用提供的徽章取代所有現有顯示的徽章。如果為 false 或省略，則會將提供的徽章新增到現有徽章。 **/
        override?: boolean
        /** 如果為 true，當使用者登入時會從租戶設定更新徽章顯示屬性。 **/
        update?: boolean
    }
}
[inline-code-end]

### SSO 使用者計費

SSO 使用者會根據其權限標記以不同方式計費：

- **一般 SSO 使用者**：沒有管理員或版主權限的使用者將以一般 SSO 使用者計費
- **SSO 管理員**：具有 `isAccountOwner` 或 `isAdminAdmin` 標記的使用者會被另外計費為 SSO 管理員（與一般租戶管理員相同費率）
- **SSO 版主**：具有 `isCommentModeratorAdmin` 標記的使用者會被另外計費為 SSO 版主（與一般版主相同費率）

**重要**：為了避免重複計費，系統會自動根據電子郵件地址將 SSO 使用者與一般租戶使用者和版主去重。如果 SSO 使用者與一般租戶使用者或版主具有相同的電子郵件，則不會被重複計費。

### 存取控制

使用者可以被分到不同群組。這就是 `groupIds` 欄位的用途，且為選用。

### @ 提及

預設情況下，當輸入 `@` 字元時，`@mentions` 會使用 `username` 來搜尋其他 SSO 使用者。如果使用 `displayName`，當 `displayName` 有匹配時，符合 `username` 的結果將被忽略，而 `@mention` 的搜尋結果會使用 `displayName`。

### 訂閱

在 FastComments 中，使用者可以透過在留言小工具中點擊鈴鐺圖示並選擇訂閱來訂閱頁面。

對於一般使用者，我們會根據他們的通知設定發送通知電子郵件。

對於 SSO 使用者，我們為了向後相容將此行為拆分。只有當您將 `optedInSubscriptionNotifications` 設為 `true` 時，使用者才會收到這些額外的訂閱通知電子郵件。

### 徽章

您可以使用 `badgeConfig` 屬性為 SSO 使用者指定徽章。徽章是在留言中顯示於使用者名稱旁的視覺指標。

- `badgeIds` - 指派給使用者的徽章 ID 陣列。這些必須是您在 FastComments 帳戶中建立的有效徽章 ID。最多限制為 30 個徽章。
- `override` - 如果為 true，留言上所有現有顯示的徽章將被提供的徽章取代。如果為 false 或省略，提供的徽章將會新增到現有徽章中。
- `update` - 如果為 true，當使用者登入時徽章的顯示屬性會從租戶設定更新。

---