FastComments 提供一個易於使用的 SSO 解決方案。使用基於 HMAC 的整合更新使用者資訊就像讓使用者載入含有更新 payload 的頁面一樣簡單。

然而，可能會希望在該流程之外管理使用者，以提高應用程式的一致性。

SSO 使用者 API 提供一種對我們稱為 SSOUsers 的物件進行 CRUD 的方式。這些物件與一般 Users 不同，為了型別安全而分開保存。

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
    isAccountOwner?: boolean // 管理員權限 - 設定此標記的 SSO 使用者會被當作 SSO 管理員計費（與一般 SSO 使用者分開）
    isAdminAdmin?: boolean // 管理員權限 - 設定此標記的 SSO 使用者會被當作 SSO 管理員計費（與一般 SSO 使用者分開）
    isCommentModeratorAdmin?: boolean // 版主權限 - 設定此標記的 SSO 使用者會被當作 SSO 版主計費（與一般 SSO 使用者分開）
    /** 若為 null，將不會對該使用者套用存取控制。若為空陣列，該使用者將無法看到任何頁面或在 @ 提及其他使用者。 **/
    groupIds?: string[] | null
    createdFromSimpleSSO?: boolean
    /** 不允許其他使用者在該使用者的個人檔案上查看其活動（包括留言）。預設為 true，以預設提供安全的個人檔案。 **/
    isProfileActivityPrivate?: boolean
    /** 不允許其他使用者在該使用者的個人檔案上留下留言，或查看現有的個人檔案留言。預設為 false。 **/
    isProfileCommentsPrivate?: boolean
    /** 不允許其他使用者發送私人訊息給該使用者。預設為 false。 **/
    isProfileDMDisabled?: boolean
    karma?: number
    /** 使用者徽章的選用設定。 **/
    badgeConfig?: {
        /** 指派給使用者的徽章 ID 陣列。限 30 個徽章。順序會保留。這些為在所有頁面可見的全域徽章。 **/
        badgeIds: string[]
        /** 只針對當前頁面（urlId）作用的徽章 ID 陣列。這些徽章僅在被指定的頁面上顯示。 **/
        pageBadgeIds?: string[]
        /** 若為 true，則以提供的徽章替換所有現有顯示的徽章。全域與頁面範圍的徽章獨立覆蓋。若為 false，則會新增到現有徽章。 **/
        override?: boolean
        /** 若為 true，會依租戶設定更新徽章顯示屬性。 **/
        update?: boolean
    }
}
[inline-code-end]

### SSO 使用者計費

SSO 使用者的計費會根據其權限標記而不同：

- **Regular SSO Users**：沒有管理或版主權限的使用者會以一般 SSO 使用者計費
- **SSO Admins**：具有 `isAccountOwner` 或 `isAdminAdmin` 標記的使用者會被另外計費為 SSO 管理員（與一般租戶管理員相同費率）
- **SSO Moderators**：具有 `isCommentModeratorAdmin` 標記的使用者會被另外計費為 SSO 版主（與一般版主相同費率）

**重要**：為避免重複計費，系統會自動根據電子郵件地址對 SSO 使用者與一般租戶使用者及版主進行去重。如果 SSO 使用者的電子郵件與一般租戶使用者或版主相同，將不會重複計費。

### 存取控制

使用者可以被劃分為群組。這就是 `groupIds` 欄位的用途，且為選用。

### @提及

預設情況下，當輸入 `@` 字元時，`@mentions` 將使用 `username` 搜尋其他 SSO 使用者。如果使用 `displayName`，當有符合 `displayName` 的結果時，會忽略符合 `username` 的結果，且 `@mention` 的搜尋結果將使用 `displayName`。

### 訂閱

在 FastComments 中，使用者可以透過在留言元件中點擊鈴鐺圖示並點選訂閱，來訂閱某個頁面。

對於一般使用者，我們會根據他們的通知設定發送通知電子郵件。

對於 SSO 使用者，為了向後相容我們將此行為做了區分。僅當您將 `optedInSubscriptionNotifications` 設為 `true` 時，使用者才會收到這些額外的訂閱通知電子郵件。

### 徽章

您可以使用 `badgeConfig` 屬性為 SSO 使用者指派徽章。徽章是顯示在使用者名稱旁的視覺標示。

- `badgeIds` - 指派給使用者的徽章 ID 陣列。這些為在所有頁面可見的全域徽章。必須是您 FastComments 帳戶中已建立的有效徽章 ID。限制為 30 個徽章。
- `pageBadgeIds` - 選用的徽章 ID 陣列，僅限於當前頁面（`urlId`）。這些徽章只會在指派它們的頁面上顯示。不同頁面對同一使用者可以有不同的頁面範圍徽章。
- `override` - 若為 true，所有現有顯示的徽章將被提供的徽章取代。全域與頁面範圍的徽章獨立覆蓋 — 覆蓋全域徽章不會影響頁面範圍徽章，反之亦然。若為 false 或省略，則會將提供的徽章新增到任何現有徽章上。
- `update` - 若為 true，使用者登入時會從租戶設定更新徽章顯示屬性。