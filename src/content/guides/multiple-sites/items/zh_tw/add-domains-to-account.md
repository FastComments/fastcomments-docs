FastComments 會驗證對您帳戶的請求，以確保它們來自您的網站。這就是為什麼我們需要知道您想要安裝 FastComments 的網站（或多個網站）。

FastComments 支援透過網域以及子網域進行驗證。

以網站 `https://example.com` 為例。在此情況下，"`example.com`" 為網域。`example.com` 同時支援 `example.com` 與 `www.example.com`。我們將 "www" 稱為「子網域」。

例如：

- 若只允許 `blog.example.com`：
  - 將 `blog.example.com` 新增至您的網域清單。
- 若允許 `www.example.com`、`somesite.example.com` 與 `example.com`：
  - 將 `example.com` 新增至您的網域清單。
  - 這會被計算為 **一個網域** 與您的帳戶關聯。
- 您現在也可以新增通配符子網域，例如 *myname.vercel.app*。
  - 這同樣會被計算為 **一個網域** 與您的帳戶關聯。

如果您使用的是部落格平台，且取得了子網域，您需要將 **完整的網域（含子網域）** 新增至您的帳戶，例如：`cats.blogger.com`。

您可以透過前往 `My Domains` 頁面並在底部點擊 `Add a Domain` 來為帳戶新增網域：

[app-screenshot-start url='/auth/my-account/configure-domains'; selector = '.content'; alt='帳戶上列出域名的「我的域名」頁面，底部有「新增域名」按鈕'; title='我的域名頁面' app-screenshot-end]

在試用期間，**當請求來自該網域時，系統會自動將網域新增至您的帳戶**。然而，試用期結束後，為了安全性，必須手動明確新增。系統會在此自動行為發生時發送電子郵件通知您。

您 **不需要** 為本機開發新增 `localhost`——預設已允許。

#### 透過 API

也可以透過 [DomainConfigs API](/guide-api.html#domain-config-structure) 來新增與設定網域。