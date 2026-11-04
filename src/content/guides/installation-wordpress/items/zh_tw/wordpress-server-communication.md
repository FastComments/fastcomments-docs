為了讓外掛正常運作，一個 token 會儲存在您的 WordPress 資料庫以及您的 FastComments 帳戶中。當外掛向我們的伺服器發送請求時，它會提供此 token。

您可以在[這裡](https://fastcomments.com/auth/my-account/manage-data/integrations)查看所有已授權存取您 FastComments 帳戶的整合。

所有通訊均透過 HTTPS 進行。

所有通訊都是從您的 WordPress 伺服器*向外*傳送*至* FastComments.com，包括同步*回*您的 WordPress 安裝，因為它是透過 WordPress 安裝中的 [cron](https://en.wikipedia.org/wiki/Polling_(computer_science) 設定進行 [輪詢](https://developer.wordpress.org/plugins/cron/))來實作的。