[related-parameter-start name = 'sso'; type = 'FastCommentsSSO'; typeLink = 'https://github.com/FastComments/fastcomments-typescript/blob/main/src/fast-comments-comment-widget-config.ts#L1' related-parameter-end]

FastComments 安全 SSO 使用 HMAC-SHA256 加密作為實現 SSO 的機制。我們將先介紹整體架構，提供範例以及詳細步驟。

另外還有一些關於從其他具有類似 SSO 機制的供應商遷移的文件，以及兩者之間的差異。

流程如下所示：

<div class="screenshot white-bg">
    <div class="title">安全 SSO 流程</div>
    <img class="screenshot-image" src="/images/secure-sso-diagram.svg" alt="安全 SSO 圖表" />
</div>

由於安全 SSO 涉及全端開發，目前 Java/Spring、NodeJS/Express 以及原生 PHP 的完整可運行程式碼範例已發佈在 <a href="https://github.com/FastComments/fastcomments-code-examples/tree/master/sso" target="_blank">GitHub</a> 上。

雖然我們在 NodeJS 範例中使用 ExpressJS，在 Java 範例中使用 Spring，但實現 FastComments SSO 在這些執行環境中不需要任何框架/函式庫 - 原生的加密套件即可運作。

使用 FastComments SSO 您不需要撰寫任何新的 API 端點。只需使用您的密鑰加密使用者資訊，並將承載資料傳遞給留言小工具即可。

#### 取得您的 API 密鑰

您的 API 密鑰可以從<a href="https://fastcomments.com/auth/my-account/api-secret" target="_blank">此頁面</a>取得。您也可以透過前往「我的帳戶」，點擊 API/SSO 圖塊，然後點擊「Get API Secret Key」來找到此頁面。

#### 留言小工具參數

留言小工具的高階 API 文件可以在<a href="https://github.com/FastComments/fastcomments-typescript/blob/main/src/fast-comments-comment-widget-config.ts#L1" target="_blank">這裡</a>找到。

讓我們更詳細地了解這些參數的含義。

留言小工具接受一個設定物件 - 如果您使用 FastComments 傳遞您的客戶 ID（稱為 tenantId），您已經在傳遞此物件了。

要啟用 SSO，請傳遞一個新的「sso」物件，該物件必須包含以下參數。這些值應在伺服器端產生。

- userDataJSONBase64: 使用者的資料，以 JSON 格式表示，然後進行 Base64 編碼。
- verificationHash: 由 UNIX_TIME_MILLIS + userDataJSONBase64 建立的 HMAC-SHA256 雜湊值。
- timestamp: Epoch 時間戳，以**毫秒**為單位。不得為未來的時間，或超過兩天前。
- loginURL: 留言小工具可以顯示用於登入使用者的 URL。
- logoutURL: 留言小工具可以顯示用於登出使用者的 URL。
- loginCallback: 當提供此項而非登入 URL 時，留言小工具在點擊登入按鈕時將呼叫的函式。
- logoutCallback: 當提供此項而非登出 URL 時，留言小工具在點擊登出按鈕時將呼叫的函式。

[code-example-start config = {sso: { userDataJSONBase64: '...', verificationHash: '...', timestamp: Date.now(), loginURL: 'https://example.com/login', logoutURL: 'https://example.com/logout', loginCallback: function() { console.log('Log the user in here...'); }, logoutCallback: function() { console.log('Log the user out here...') } }}; linesToHighlight = [6, 7, 8, 9, 10, 11, 12]; title = '安全 SSO 客戶端程式碼'; isFunctional = false; code-example-end]

#### 使用者物件

使用者物件包含以下架構：
[inline-code-attrs-start title = '使用者物件'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface SSOUser {
    /** 必填。最多 1k 個字元。 **/
    id: string;
    /** 必填。最多 1k 個字元。注意：必須是唯一的。 **/
    email: string;
    /** 必填。最多 1k 個字元。注意：使用者名稱不能是電子郵件。不必是唯一的。 **/
    username: string;
    /** 選填。URL 最多 3k 個字元。預設是根據電子郵件從 gravatar 取得。支援 64 編碼的圖片，此情況下限制為 50k 個字元。 **/ 
    avatar?: string;
    /** 選填。預設為 false。 **/
    optedInNotifications?: boolean;
    /** 選填。預設為 false。 **/
    optedInSubscriptionNotifications?: boolean;
    /** 選填。最多 100 個字元。此標籤將顯示在其名稱旁邊。適用時預設為 Administrator/Moderator。 **/
    displayLabel?: string;
    /** 選填。最多 500 個字元。將顯示此名稱而非使用者名稱。 **/
    displayName?: string;
    /** 選填。最多 2k 個字元。使用者的名稱將連結到此。 **/
    websiteUrl?: string;
    /** 選填。每位使用者最多 100 個群組。群組 ID 不得超過 50 個字元。 **/
    groupIds?: string[];
    /** 選填。將該使用者標記為管理員。 **/
    isAdmin?: boolean;
    /** 選填。將該使用者標記為版主。 **/
    isModerator?: boolean;
    /** 選填，預設為 true。設為 false 以啟用使用者個人檔案中的「活動」分頁。 **/
    isProfileActivityPrivate?: boolean;
    /** 選填，預設為 false。設為 true 以停用個人檔案留言。 **/
    isProfileCommentsPrivate?: boolean;
    /** 選填，預設為 false。設為 true 以停用傳送私人訊息給此使用者。 **/
    isProfileDMDisabled?: boolean;
    /** 使用者徽章的選填設定。 **/
    badgeConfig?: {
        /** 要指派的全球徽章 ID 陣列。限制為 30 個徽章。順序會被保留。 **/
        badgeIds: string[];
        /** 限於當前頁面 (urlId) 的徽章 ID 陣列。僅在指派的頁面上顯示。 **/
        pageBadgeIds?: string[];
        /** 若為 true，則取代現有顯示的徽章。全球與頁面範圍的徽章會分別被覆寫。 **/
        override?: boolean;
        /** 若為 true，則從租戶設定更新徽章顯示屬性。 **/
        update?: boolean;
    };
}
[inline-code-end]

#### 版主與管理員

對於管理員和版主，請在 `SSOUser` 物件中傳遞相應的 `isAdmin` 或 `isModerator` 旗標。

#### 通知

要啟用或停用通知，請分別將 `optedInNotifications` 的值設為 `true` 或 `false`。當使用者首次載入帶有此 SSO 承載資料值的頁面時，他們的通知設定將會被更新。

此外，如果您希望使用者收到其訂閱頁面上活動的通知電子郵件（而不僅僅是應用程式內通知），請將 `optedInSubscriptionNotifications` 設為 `true`。

#### VIP 使用者與特殊標籤

您可以透過使用選填的「displayLabel」欄位，在使用者名稱旁邊顯示特殊標籤。

#### 未驗證的使用者

要表示未驗證的使用者，只需不填入 userDataJSONBase64、verificationHash 或 timestamp。並提供一個 loginURL。

這些使用者將無法留言，而是會看到登入訊息（訊息、連結或按鈕，取決於設定）。

#### 序列化與雜湊使用者資料的直接範例

更多詳細資訊的範例請見<a href="https://github.com/FastComments/fastcomments-code-examples/blob/master/sso/node-express/routes/index.js#L23" target="_blank">這裡</a> (js)、<a href="https://github.com/FastComments/fastcomments-code-examples/blob/master/sso/java-springboot/src/main/java/com/winricklabs/ssodemo/DemoController.java#L68" target="_blank">這裡</a> (java) 以及<a href="https://github.com/FastComments/fastcomments-code-examples/blob/master/sso/php/server.php#L27" target="_blank">這裡</a> (php)。

我們了解任何整合都可能是複雜且痛苦的過程。請隨時聯絡您的業務代表或使用<a href="https://fastcomments.com/auth/my-account/help" target="_blank">支援頁面</a>。