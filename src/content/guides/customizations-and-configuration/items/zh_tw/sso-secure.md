[related-parameter-start name = 'sso'; type = 'FastCommentsSSO'; typeLink = 'https://github.com/FastComments/fastcomments-typescript/blob/main/src/fast-comments-comment-widget-config.ts#L1' related-parameter-end]

FastComments Secure SSO 使用 HMAC-SHA256 加密作為實作 SSO 的機制。首先我們會介紹整體架構、提供範例，以及詳細步驟。

也有一些關於從具有類似 SSO 機制的其他供應商遷移的文件，以及差異說明。

流程如下所示：

<div class="screenshot white-bg">
    <div class="title">Secure SSO Flow</div>
    <img class="screenshot-image" src="/images/secure-sso-diagram.svg" alt="Secure SSO Diagram" />
</div>

由於 Secure SSO 涉及全端開發，目前 Java/Spring、NodeJS/Express 與純 PHP 的完整工作範例都在 <a href="https://github.com/FastComments/fastcomments-code-examples/tree/master/sso" target="_blank">GitHub</a> 上。

雖然我們在 NodeJS 範例中使用 ExpressJS，Java 範例中使用 Spring，但在這些執行環境中實作 FastComments SSO 並不需要任何框架/函式庫 — 原生的加密套件即可運作。

使用 FastComments SSO 時不需要撰寫任何新的 API 端點。只要使用您的密鑰加密使用者資訊並將 payload 傳給評論元件即可。

#### 取得您的 API 秘密金鑰

您的 API Secret 可從 <a href="https://fastcomments.com/auth/my-account/api-secret" target="_blank">此頁面</a> 取得。您也可以在「My Account」中點選 API/SSO 方塊，然後點選「Get API Secret Key」來找到此頁面。

#### 評論元件參數

評論元件的高階 API 文件可在 <a href="https://github.com/FastComments/fastcomments-typescript/blob/main/src/fast-comments-comment-widget-config.ts#L1" target="_blank">這裡</a> 找到。

以下更詳細說明這些參數的含義。

評論元件接收一個設定物件 — 如果您已使用 FastComments 傳遞您的客戶 ID（稱為 tenantId），您會已經在傳遞這個物件。

要啟用 SSO，傳入一個新的 "sso" 物件，該物件必須具有以下參數。這些值應在伺服器端產生。

- userDataJSONBase64: 使用者的資料（JSON 格式），接著以 Base64 編碼。
- verificationHash: 由 UNIX_TIME_MILLIS + userDataJSONBase64 製作的 HMAC-SHA256 雜湊。
- timestamp: 時間戳記，單位為 **毫秒**。不得為未來時間，且不得早於超過兩天前。
- loginURL: 評論元件可顯示用來讓使用者登入的 URL。
- logoutURL: 評論元件可顯示用來讓使用者登出的 URL。
- loginCallback: 若提供此函式而非 login URL，評論元件在點擊登入按鈕時會呼叫此函式。
- logoutCallback: 若提供此函式而非 logout URL，評論元件在點擊登出按鈕時會呼叫此函式。

[code-example-start config = {sso: { userDataJSONBase64: '...', verificationHash: '...', timestamp: Date.now(), loginURL: 'https://example.com/login', logoutURL: 'https://example.com/logout', loginCallback: function() { console.log('Log the user in here...'); }, logoutCallback: function() { console.log('Log the user out here...') } }}; linesToHighlight = [6, 7, 8, 9, 10, 11, 12]; title = 'Secure SSO Client Code'; isFunctional = false; code-example-end]

#### 使用者物件

The User object contains the following schema:
[inline-code-attrs-start title = '使用者物件'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface SSOUser {
    /** 必填。最多 1k 字元。 **/
    id: string;
    /** 必填。最多 1k 字元。注意：必須唯一。 **/
    email: string;
    /** 必填。最多 1k 字元。注意：使用者名稱不得為電子郵件。無需唯一。 **/
    username: string;
    /** 選填。URL 最多 3k 字元。預設為根據 email 的 gravatar。支援 64 編碼的圖片，在此情況下限制為 50k 字元。 **/ 
    avatar?: string;
    /** 選填。預設為 false。 **/
    optedInNotifications?: boolean;
    /** 選填。預設為 false。 **/
    optedInSubscriptionNotifications?: boolean;
    /** 選填。最多 100 字元。此標籤會顯示在使用者名稱旁。視情況預設為 管理者/版主。 **/
    displayLabel?: string;
    /** 選填。最多 500 字元。此欄位會取代使用者名稱顯示。 **/
    displayName?: string;
    /** 選填。最多 2k 字元。使用者名稱會連結到此 URL。 **/
    websiteUrl?: string;
    /** 選填。每位使用者最多可有 100 個群組。群組 id 最長不可超過 50 字元。 **/
    groupIds?: string[];
    /** 選填。標示該使用者為管理者。 **/
    isAdmin?: boolean;
    /** 選填。標示該使用者為版主。 **/
    isModerator?: boolean;
    /** 選填，預設為 true。設為 false 可啟用使用者個人資料中的「activity」分頁。 **/
    isProfileActivityPrivate?: boolean;
    /** 選填，預設為 false。設為 true 以停用個人資料留言。 **/
    isProfileCommentsPrivate?: boolean;
    /** 選填，預設為 false。設為 true 以停用對此使用者的直接訊息。 **/
    isProfileDMDisabled?: boolean;
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

More details as an examples <a href="https://github.com/fastcomments/fastcomments-code-examples/blob/master/sso/nodejs/routes/index.js#L26" target="_blank">here</a> (js), <a href="https://github.com/FastComments/fastcomments-code-examples/blob/master/sso/java/src/main/java/com/winricklabs/ssodemo/DemoController.java#L54" target="_blank">here</a> (java) and <a href="https://github.com/fastcomments/fastcomments-code-examples/blob/master/sso/php/server.php#L27" target="_blank">here</a> (php).

We understand that any integration can be a complicated and painful process. Don't hesitate to reach out to your representative or use the <a href="https://fastcomments.com/auth/my-account/help" target="_blank">support page</a>.

---