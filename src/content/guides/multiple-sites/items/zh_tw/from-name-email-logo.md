有時候 FastComments 必須透過電子郵件聯絡您的使用者，特別是當您沒有使用 Secure SSO 時。

這包括在使用者首次留言時驗證他們的帳戶或活動。FastComments  
也會在有回覆他們留言時寄送通知給他們。

當 FastComments 寄送電子郵件給您的使用者時，我們預設會使用的寄件名稱與電子郵件為 `FastComments Robot` 與 `noreply@fastcomments.com`。

我們也會在這些郵件的頁尾使用我們自己的標誌。

如果您有 FastComments Flex 或 Pro，這些都可以在「我的網域」頁面上以每個網域為單位自訂：

[app-screenshot-start url='/auth/my-account/configure-domains'; selector = '.content form'; title='Customizing From Name, Email, and Logo' app-screenshot-end]

當在郵件中自訂顯示的 logo 時，請確保您上傳的圖像尺寸就是您希望在郵件頁尾顯示的尺寸。

### 當自訂 `From Domain` 時

如果您自訂 `From Domain`，電子郵件服務商與用戶端需要知道 FastComments 已被授權代表您發送郵件。否則，  
僅設定 `From Domain` 而沒有遵照以下步驟，很可能會導致郵件被歸類為垃圾郵件。

#### 1. 設定 SPF

為了允許 FastComments 安全地以您的網域發送郵件，請務必新增一筆允許我們這麼做的 SPF 紀錄。

請確保有 SPF 紀錄允許 `mail.fastcomments.com` 與 `sib.fastcomments.com` 以您的網域發送郵件。

關於如何執行的更多資訊請見：https://mailtrap.io/blog/multiple-spf-records/

#### 2. 設定 DKIM

除了 SPF 之外，您也應該設定 DKIM。當您的 DNS 設定準備好後，您可以在網域設定頁面按一下「顯示進階」以顯示每個網域的 DKIM 設定。

您也可以透過 [呼叫 API](/guide-api.html#domain-config-structure) 來設定 DKIM。

### 取消訂閱連結

使用 SSO 時，郵件與通知中使用的取消訂閱功能可以透過 [透過 DomainConfigs API](/guide-api.html#domain-config-structure) 進行自訂。