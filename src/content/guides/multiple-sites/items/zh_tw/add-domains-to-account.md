FastComments 會驗證發向您帳戶的請求，以確認這些請求來自您網站。這就是為什麼
我們需要知道您要在哪個網站（或哪些網站）上安裝 FastComments。

FastComments 支援以網域以及子網域進行驗證。

我們以網站 `https://example.com` 為例。在此情況下，`example.com` 是網域。`example.com` 同時支援 `example.com` 與 `www.example.com`。我們會將「www」稱為「子網域」。

For Example:

- 只允許 `blog.example.com`：
  - 將 `blog.example.com` 新增到您的網域清單。
- 若要允許 `www.example.com`、`somesite.example.com` 與 `example.com`：
  - 將 `example.com` 新增到您的網域清單。
  - 這會被計算為您的帳戶關聯 **一個網域**。
- 您現在也可以新增萬用字元子網域，例如 *myname.vercel.app. 
  - 這會被計算為您的帳戶關聯 **一個網域**。

如果您使用的是部落格平台，且獲分配了一個子網域，您會想要
將 **包含子網域的完整網域** 新增到您的帳戶，例如：`cats.blogger.com`

We can add domains to our account by visiting the `My Domains` page and clicking `Add a Domain` at the bottom:

[app-screenshot-start url='/auth/my-account/configure-domains'; selector = '.content'; title='The My Domains Page' app-screenshot-end]

在試用期間，**當請求來自該等網域時，網域會自動新增至您的帳戶**。然而，
超過此期間後，出於安全理由必須明確新增。當此自動行為發生時，您應會收到電子郵件通知。

您**不**需要為本機開發新增 `localhost` — 它預設被允許。

#### 透過 API

網域也可以新增與設定 [透過 DomainConfigs API](/guide-api.html#domain-config-structure)。