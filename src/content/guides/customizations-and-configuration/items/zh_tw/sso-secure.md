[related-parameter-start name = 'sso'; type = 'FastCommentsSSO'; typeLink = 'https://github.com/FastComments/fastcomments-typescript/blob/main/src/fast-comments-comment-widget-config.ts#L1' related-parameter-end]

FastComments Secure SSO 使用 HMAC-SHA256 作為實作 SSO 的加密機制。首先我們會介紹整體架構、提供範例，以及詳細步驟。

也有一些關於從其他使用類似 SSO 機制的提供者遷移的文件，以及差異說明。

流程如下：

<div class="screenshot white-bg">
    <div class="title">安全 SSO 流程</div>
    <img class="screenshot-image" src="/images/secure-sso-diagram.svg" alt="安全 SSO 圖示" />
</div>

由於 Secure SSO 涉及全端開發，完整的可執行範例程式碼（Java/Spring、NodeJS/Express、以及原生 PHP）目前都放在 <a href="https://github.com/FastComments/fastcomments-code-examples/tree/master/sso" target="_blank">GitHub</a>。

雖然我們在 NodeJS 範例中使用 ExpressJS，在 Java 範例中使用 Spring，但在這些執行環境中實作 FastComments SSO 並不需要其它框架/函式庫 — 原生的 crypto 套件就足夠了。

使用 FastComments SSO 不需要你新增任何 API 端點。只要用你的 secret key 加密使用者資訊，並將負載傳給留言元件即可。

#### 取得你的 API Secret Key

你的 API Secret 可從 <a href="https://fastcomments.com/auth/my-account/api-secret" target="_blank">此頁面</a> 取得。你也可以從 我的帳戶（My Account）→ 點選 API/SSO 區塊 → 然後點選「取得 API Secret Key」找到此頁面。

#### 留言元件參數

留言元件的高階 API 文件可在 <a href="https://github.com/FastComments/fastcomments-typescript/blob/main/src/fast-comments-comment-widget-config.ts#L1" target="_blank">這裡</a> 找到。

以下更詳細說明這些參數的含義。

留言元件接受一個設定物件 — 如果你已經在使用 FastComments 並傳入 tenantId（稱為 customer id），你應該已經在傳遞該物件。

要啟用 SSO，傳入一個新的 "sso" 物件，其必須包含下列參數。這些值應該在伺服器端產生。

- userDataJSONBase64: 使用者的資料（JSON 格式），然後進行 Base64 編碼。
- verificationHash: 由 UNIX_TIME_MILLIS + userDataJSONBase64 使用 HMAC-SHA256 所產生的雜湊。
- timestamp: 時間戳記，單位為 **毫秒**。不得為未來時間，且不得早於超過兩天前。
- loginURL: 留言元件可顯示用來登入使用者的 URL。
- logoutURL: 留言元件可顯示用來登出使用者的 URL。
- loginCallback: 當提供此函式而非 login URL 時，留言元件在點擊登入按鈕時會呼叫該函式。
- logoutCallback: 當提供此函式而非 logout URL 時，留言元件在點擊登出按鈕時會呼叫該函式。

[code-example-start config = {sso: { userDataJSONBase64: '...', verificationHash: '...', timestamp: Date.now(), loginURL: 'https://example.com/login', logoutURL: 'https://example.com/logout', loginCallback: function() { console.log('Log the user in here...'); }, logoutCallback: function() { console.log('Log the user out here...') } }}; linesToHighlight = [6, 7, 8, 9, 10, 11, 12]; title = 'Secure SSO Client Code'; isFunctional = false; code-example-end]

#### The User Object

The User object contains the following schema:
[inline-code-attrs-start title = '使用者物件'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface SSOUser {
    /** 必填。最多 1k 字元。 **/
    id: string;
    /** 必填。最多 1k 字元。註：必須唯一。 **/
    email: string;
    /** 必填。最多 1k 字元。註：使用者名稱不能是 email。不必唯一。 **/
    username: string;
    /** 選填。URL 最多 3k 字元。預設根據 email 從 gravatar 取得。支援 64 編碼的圖片，在此情況下限制為 50k 字元。 **/ 
    avatar?: string;
    /** 選填。預設為 false。 **/
    optedInNotifications?: boolean;
    /** 選填。預設為 false。 **/
    optedInSubscriptionNotifications?: boolean;
    /** 選填。最多 100 字元。此標籤會顯示在使用者名稱旁邊。若適用，預設為 Administrator/Moderator。 **/
    displayLabel?: string;
    /** 選填。最多 500 字元。此內容會取代 username 顯示。 **/
    displayName?: string;
    /** 選填。最多 2k 字元。使用者名稱的連結會指向此處。 **/
    websiteUrl?: string;
    /** 選填。每位使用者最多 100 個群組。群組 id 最長不得超過 50 個字元。 **/
    groupIds?: string[];
    /** 選填。將使用者標示為管理員。 **/
    isAdmin?: boolean;
    /** 選填。將使用者標示為版主。 **/
    isModerator?: boolean;
    /** 選填，預設為 true。設為 false 以啟用使用者個人檔案中的「activity」分頁。 **/
    isProfileActivityPrivate?: boolean;
    /** 選填，預設為 false。設為 true 以停用個人檔案留言。 **/
    isProfileCommentsPrivate?: boolean;
    /** 選填，預設為 false。設為 true 以停用直接訊息功能。 **/
    isProfileDMDisabled?: boolean;
    /** 使用者徽章的選填設定。 **/
    badgeConfig?: {
        /** 指派的全域徽章 ID 陣列。限制為 30 個徽章。順序會被保留。 **/
        badgeIds: string[];
        /** 指派給當前頁面 (urlId) 的徽章 ID 陣列。僅在被指派的頁面顯示。 **/
        pageBadgeIds?: string[];
        /** 若為 true，會取代現有顯示的徽章。全域與頁面範圍的徽章分別獨立覆寫。 **/
        override?: boolean;
        /** 若為 true，會從租戶設定更新徽章顯示屬性。 **/
        update?: boolean;
    };
}
[inline-code-end]

#### Moderators and Administrators

For admins and moderators, pass the respective `isAdmin` or `isModerator` flags in the `SSOUser` object.

#### Notifications

To enable or disable notifications, set the value of `optedInNotifications` to `true` or `false` respectively. The first time the user loads the page with this value in the SSO payload, their notification settings will be updated.

Additionally, if you want users to receive notification emails for activity on pages they subscribed to (as opposed to just in-app notifications), then set `optedInSubscriptionNotifications` to `true`.

#### VIP Users & Special Labels

You can display a special label next to the user's name by using the optional "displayLabel" field.

#### Unauthenticated users

To represent an unauthenticated user, simply do not populate userDataJSONBase64, verificationHash, or timestamp. Provide a loginURL.

These users will not be able to comment, and instead will be presented with a login message (message, link, or button, depending on configuration).

#### Direct Examples for Serializing and Hashing User Data

More details as an examples <a href="https://github.com/fastcomments/fastcomments-code-examples/blob/master/sso/nodejs/routes/index.js#L26" target="_blank">here</a> (js), <a href="https://github.com/fastcomments/fastcomments-code-examples/blob/master/sso/java/src/main/java/com/winricklabs/ssodemo/DemoController.java#L54" target="_blank">here</a> (java) and <a href="https://github.com/fastcomments/fastcomments-code-examples/blob/master/sso/php/server.php#L27" target="_blank">here</a> (php).

We understand that any integration can be a complicated and painful process. Don't hesitate to reach out to your representative or use the <a href="https://fastcomments.com/auth/my-account/help" target="_blank">support page</a>.