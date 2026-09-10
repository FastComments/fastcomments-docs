---
跟隨在 `localhost` 上的步驟與在正式環境相同。確保已設定正式環境的網域與 API 密鑰。

首先，前往 [Webhooks 管理](https://fastcomments.com/auth/my-account/manage-data/webhooks)。此頁面可於 **管理資料 -> Webhooks** 取得。

此頁面會列出您帳號中的所有 webhook：

[app-screenshot-start url='/auth/my-account/manage-data/webhooks'; selector = '.content'; alt='Webhooks 管理頁面列出每個 webhook 及其 URL、事件、域名、方法、狀態與排隊事件數量'; title='Webhooks 列表'; cacheBuster = 'v4' app-screenshot-end]

點擊 **新增 Webhook** 以新增。每個 webhook 包含一個 URL、一個評論事件（建立、更新或刪除）、一個域名，以及一個 HTTP 方法：

[app-screenshot-start url='/auth/my-account/manage-data/webhooks/new'; selector = '.content'; alt='新增 webhook 表單，包含 URL、事件、域名與 HTTP 方法欄位，並有「發送測試有效負載」按鈕'; title='新增 Webhook'; cacheBuster = 'v4' app-screenshot-end]

每個 webhook 皆獨立傳送。您可以將相同的事件發送至多個端點，且範圍設定為 **All Domains** 的 webhook 會接收所有域名的評論，即使同一事件在特定域名已存在 webhook。相同的 URL、事件與域名不可重複新增。

在儲存之前，點擊 **Send Test Payload** 以檢查端點是否接受已簽名的請求。請參閱下一節「測試」以取得詳細資訊。

在列表中，您可以編輯、停用、重新啟用或刪除 webhook。停用會保留排隊的事件，直到 webhook 重新啟用；刪除則會丟棄這些事件。

Webhook 也可以透過 API 建立，例如使用 Zapier。這些會在同一列表中顯示，來源標示為 **API**。請參閱「透過 API 管理 Webhook」以了解更多。

---