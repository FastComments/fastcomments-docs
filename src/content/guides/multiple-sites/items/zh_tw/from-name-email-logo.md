有時 FastComments 需要透過電子郵件聯絡你的使用者，特別是當你沒有使用安全 SSO 時。

例如，當使用者首次留言時，我們會寄送電子郵件來驗證他們的帳號或活動。FastComments 也會寄送回覆通知給留言的使用者。

當 FastComments 寄信給你的使用者時，我們會使用預設的寄件人名稱與電子郵件 `FastComments Robot` 和 `noreply@fastcomments.com`。

在這些電子郵件的頁尾，我們也會使用我們自己的商標圖示。

如果你有 FastComments Flex 或 Pro，可以在「我的網域」頁面上針對每個網域自訂這些設定：

[app-screenshot-start url='/auth/my-account/configure-domains'; selector = '.content form'; title='自訂寄件人名稱、電子郵件與標誌' app-screenshot-end]

當你自訂要在電子郵件中顯示的商標圖示時，請確保你上傳的尺寸就是你希望在電子郵件頁尾顯示的尺寸。

### When Customizing The `From Domain`

如果你自訂 `From Domain`，電子郵件提供者與用戶端需要知道 FastComments 已被授權代表你的網域發送電子郵件。否則，
只定義了 `From Domain` 而沒有遵循下列步驟，電子郵件很可能會被判為垃圾郵件。

#### 1. Setup SPF

為了允許 FastComments 以你的網域安全地發送電子郵件，請確保你新增一個 SPF 紀錄來允許我們這麼做。

請確保有 SPF 紀錄允許 `mail.fastcomments.com` 與 `sib.fastcomments.com` 代表你的網域發送郵件。

有關如何操作的更多資訊請參考： https://mailtrap.io/blog/multiple-spf-records/

#### 2. Setup DKIM

除了 SPF，你也應該設定 DKIM。當你的 DNS 設定就緒後，你可以在網域設定頁面中點選「顯示進階」來顯示每個網域的 DKIM 設定。

你也可以 [呼叫 API](/guide-api.html#domain-config-structure) 來設定 DKIM 組態。

### Unsubscribe Links

當使用 SSO 時，電子郵件與通知中使用的取消訂閱功能可以透過 [DomainConfigs API](/guide-api.html#domain-config-structure) 進行自訂。

### Email Link Obfuscation

如果你網站的網域聲譽導致通知電子郵件進入垃圾郵件夾，你可以將「查看留言」按鈕透過 `fastcomments.com` 重新導向，而不是直接連到你的頁面。郵件提供者會根據電子郵件內文中的每個連結對目的地的聲譽進行評分，所以當你的網域被標記時，裸露的連結會無論你的寄送設定多乾淨，都對垃圾郵件分數產生負面影響。

可在「我的網域」頁面的「顯示進階」中啟用此功能，位於「Email Link Obfuscation」區段。此設定為每個網域單獨設定。

啟用後，提及、回覆、新留言、訂閱頁面、個人資料留言與彙整郵件中的連結會改寫為短代碼，點擊時會重新導向到原始頁面。目的地會綁定到你的租戶：重新導向只會轉發到 host 與你所設定的網域之一相符的 URL，且代碼在 30 天後自動失效。

點擊後的使用者體驗不會改變。讀者仍會到達你的頁面，並且留言會自動捲動到畫面中。