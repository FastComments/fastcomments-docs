SSO，或稱單一登入，是一套慣例，用來讓您或您的使用者在不需建立另一組帳號的情況下使用 FastComments。

假設您不允許匿名留言，則需有帳號才能在 FastComments 留言。我們讓這個註冊流程非常簡單——使用者只要在留言時留下一個電子郵件地址即可。
不過，我們理解連這一點對某些網站來說也是額外摩擦。

我們可以透過為整個網站只使用一個登入流程來減少這種摩擦。

### How do I get it?
所有帳號類型目前都可使用 SSO。不過，SSO 使用者的最大數量會依您購買的方案而有所不同。與其他功能一樣，Pro 計畫及以上方案提供直接的開發支援。

讓我們比較各個選項，然後深入說明每個選項的細節。

### User and Comment Migrations

當從像 Disqus 這類具有 SSO 的平台遷移時，您原本就會有使用者及其留言。

留言會作為遷移的一部分被匯入，無論是透過 API、我們的匯入介面 (Import UI)，或是客服支援。如果匯入介面支援您要遷移的來源平台，則建議使用匯入介面，因為它包含錯誤處理、頭像與媒體的擷取與上傳，以及批次工作監控系統。

使用者本身會在第一次檢視留言串時自動加入。或是，他們也可以透過 API 事先加入，但這樣做沒有太多優勢。

如果留言已被匯入，而 SSO 使用者並未透過 API 手動新增，則當使用者在第一次檢視任何留言串時，留言會自動被遷移到該使用者的帳戶。之後他們就能管理、編輯和刪除自己原本所發表的留言。

自動遷移是透過電子郵件或使用者名稱進行。有些平台在匯出時不提供電子郵件，例如 Disqus，因此在這種情況下我們會退而求其次使用使用者名稱。
- 只要您在 SSO payload 中傳遞相符的使用者名稱及電子郵件，我們就會將電子郵件加入到個別的留言物件中，以便通知和提及功能能正常運作。

如果您希望同時匯入留言和使用者，請與支援團隊合作，在透過 API 匯入使用者後，將留言遷移到對應的使用者帳戶。

總結來說，遷移的最簡單路徑是：

1. 匯入留言。
   1. 如果在 `Manage Data -> Imports` 使用匯入介面，頭像和其他媒體會自動被遷移。
2. 設定 Secure 或 Simple SSO。
3. 讓遷移在使用者首次登入時自動發生。
   1. 如果使用者的留言數少於 50k，這通常只會增加不到一秒的頁面載入時間。

### WordPress Users
如果您使用我們的 <a href="https://wordpress.org/plugins/fastcomments/" target="_blank">WordPress 外掛</a>，就不需要撰寫任何程式碼！只要前往外掛的管理頁面，點選 SSO 設定，然後啟用即可。

這會帶您進入一個單按鈕的精靈，該精靈會為您建立 API 金鑰、將金鑰傳送到您的 WordPress 安裝並啟用 SSO。我們已將這些步驟整合為單一按鈕點擊以簡化流程。

請注意，如果您是第一次安裝該外掛，必須完成設定流程，才能看到包含 SSO 設定按鈕的管理頁面。

#### WordPress SSO - Moderators

請注意，目前要在使用者透過 FastComments WordPress 外掛留言時，讓「Moderator」徽章顯示在您的版主旁邊，
他們也必須在 FastComments 儀表板中被新增為版主，並且其電子郵件需經過驗證。

### Custom Integrations

對於自訂整合，有兩種選項。

### Option One - Secure SSO

使用 Secure SSO 時，FastComments 可以確認正在留言、投票與閱讀留言的使用者確實是您網站上的真實使用者。

只要您建立一個有效的 payload，使用者就會獲得無縫的留言體驗。

在 Secure SSO 中，SSO payload 是在伺服器端使用 HMAC 驗證建立，然後傳遞到客戶端的小工具（widget）。

使用 Secure SSO 時，使用者帳戶與其他 FastComments 使用者群是完全分離的。這表示如果我們有兩個合作夥伴
Company A 與 Company B，兩者都可以擁有使用者名稱為 "Bob" 的 SSO 使用者。

#### Requirements
- 基本的後端開發知識。
- 基本的祕密 API 金鑰處理知識。
- 基本的 API 開發或伺服器端渲染知識。

#### Pros
- 安全。
- 無縫的留言體驗。

#### Cons
- 需要後端開發。

#### Updating User Data

使用 Secure SSO 時，每次您傳遞 sso user payload，我們都會使用最新的資訊來更新該使用者。例如，如果
使用者原本的使用者名稱是 `X`，而您在 SSO payload 中傳入 `Y`，則他們的使用者名稱會變更為 `Y`。

如果您想用此方法移除某些值，請將它們設定為 `null`（不是 `undefined`）。

#### Secure SSO API

我們也提供與 SSO 使用者互動的 API。請參閱 [the docs](/guide-api.html#sso-user-structure)。

請注意，使用 Secure SSO 時，使用者會在頁面載入時自動在背景建立。您不需要批次匯入使用者。

### Option Two - Simple SSO

Simple SSO 的替代方案是直接把使用者資訊傳給留言小工具。

使用 Simple SSO 時並不需要提供電子郵件，但若沒有電子郵件，他們的留言將顯示為「未驗證」。

<sup>注意！</sup> 截至 2022 年初，使用 Simple SSO 的使用者名稱不需要在整個 FastComments.com 上保持唯一。

理想上，只有在您開發的平台無法提供後端存取時，才應選用 Simple SSO。

#### Requirements
- 基本的前端（client-side）開發知識。
- 至少需要知道使用者的電子郵件。

#### Pros
- 簡單。
- 所有活動仍會被驗證。
- 使用者不需輸入他們的使用者名稱或電子郵件。

#### Cons
- 比 Secure SSO 安全性較低，因為客戶端的 payload 可能被構造為任意使用者。

#### Simple SSO API

透過 Simple SSO 流程自動建立的使用者會以 `SSOUser` 物件儲存。它們可以透過 `SSOUser` API 存取與管理。請參閱 [the docs](/guide-api.html#sso-user-structure)。