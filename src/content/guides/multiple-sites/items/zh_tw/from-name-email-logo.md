有時 FastComments 必須向您的使用者發送電子郵件，尤其是當您未使用安全單一登入 (SSO) 時。

例如，當使用者首次發表評論時，需要驗證其帳號或活動。FastComments 也會在有人回覆他們的評論時發送通知。

當 FastComments 向您的使用者發送電子郵件時，我們會使用預設的寄件者名稱與電子郵件，分別為 `FastComments Robot` 與 `noreply@fastcomments.com`。

我們也會在這些電子郵件的頁腳使用我們自己的標誌。

如果您使用 FastComments Flex 或 Pro，所有這些設定都可以透過「我的網域」頁面以每個網域為單位進行自訂：

[app-screenshot-start url='/auth/my-account/configure-domains'; selector = '.content form'; alt='每個網域的電子郵件設定表單，包含寄件者名稱、寄件者電子郵件與上傳標誌欄位'; title='自訂寄件者名稱、電子郵件與標誌' app-screenshot-end]

在自訂電子郵件中顯示的標誌時，請確保您上傳的尺寸與您希望在電子郵件頁腳顯示的尺寸相同。

### 自訂 `From Domain` 時

如果您自訂 `From Domain`，電子郵件服務提供者與客戶端需要知道 FastComments 已獲授權代表您發送電子郵件。否則，僅定義 `From Domain` 而未遵循以下步驟，可能會導致電子郵件被歸類為垃圾郵件。

#### 1. 設定 SPF

為了讓 FastComments 能以您的網域安全地發送電子郵件，請確保您新增允許我們發送的 SPF 記錄。

確保有 SPF 記錄允許 `mail.fastcomments.com` 與 `sib.fastcomments.com` 以您的網域發送郵件。

更多關於如何執行此操作的資訊請參考此處：https://mailtrap.io/blog/multiple-spf-records/

#### 2. 設定 DKIM

除了 SPF，您還應該設定 DKIM。當您的 DNS 設定完成後，您可以在網域設定頁面點擊「Show Advanced」以顯示每個網域的 DKIM 設定。

您也可以[呼叫 API](/guide-api.html#domain-config-structure) 來設定 DKIM 配置。

### 取消訂閱連結

使用 SSO 時，電子郵件與通知中的取消訂閱功能可透過[DomainConfigs API](/guide-api.html#domain-config-structure) 進行自訂。

### 電子郵件連結混淆

如果您網站的網域聲譽導致通知電子郵件被歸類為垃圾郵件，您可以將「檢視評論」按鈕導向 `fastcomments.com`，而非直接連結至您的頁面。郵箱服務提供者會根據目的地的聲譽對電子郵件正文中的每個連結進行評分，因此當您的網域被標記時，裸露的連結會提升垃圾郵件分數，無論您的發送設定多麼乾淨。

在「我的網域」頁面的「Show Advanced」下的「Email Link Obfuscation」區段中啟用此功能。此設定以每個網域為單位。

啟用後，提及、回覆、新評論、已訂閱頁面、個人檔案評論與摘要電子郵件中的連結會被重新寫成短代碼，點擊後會重新導向至原始頁面。目的地綁定至您的租戶：重新導向僅會轉發至主機符合您已設定網域之一的 URL，且代碼會在 30 天後自動過期。

點擊後的體驗不會改變。讀者仍會在您的頁面上看到已捲動至評論的位置。

---