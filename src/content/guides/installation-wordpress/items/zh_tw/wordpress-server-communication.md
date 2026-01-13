為了讓外掛運作，一個 token 會儲存在您的 WordPress 資料庫以及您的 FastComments 帳戶中。當外掛向我們的伺服器發出請求時，它會提供
此 token。

您可以在 [這裡](https://fastcomments.com/auth/my-account/manage-data/integrations) 檢視所有授權至您 FastComments 帳戶的整合。

所有通訊皆透過 HTTPS 進行。

所有通訊都是從您的 WordPress 伺服器 *向外* 發送至 FastComments.com，包含同步 *回* 您的 WordPress 安裝，因為該同步是實作
via [polling](https://en.wikipedia.org/wiki/Polling_(computer_science)) from a [cron](https://developer.wordpress.org/plugins/cron/) setup in your WordPress installation.