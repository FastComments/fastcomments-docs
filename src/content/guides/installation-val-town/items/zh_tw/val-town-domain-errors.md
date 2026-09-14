---
一旦關閉 `demo` 租戶，元件可能會因授權錯誤而無法載入。這是因為 FastComments 不知道它應該允許您的帳戶在該域名上使用。

[前往此處將您的網站新增至您的帳戶。](https://fastcomments.com/auth/my-account/configure-domains)

Val Town 值得再看一次，因為一個 val 可能可以透過多個主機名稱存取：

- 每個 HTTP val 都有一個較長的預設端點，`<org>--<id>.web.val.run`。
- 申請自訂子域名會新增 `<name>.val.run`。
- 一個[自訂域名](https://docs.val.town/vals/http/custom-domains/)會再新增一個。
- 分支會有自己的 URL。

將您實際提供元件服務的所有主機名稱都加入。若在設定完成後再申請子域名，也請將其加入，否則元件會在舊的 URL 上運作，而在新的 URL 上失敗。
---