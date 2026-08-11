---
對於 `localhost` 請遵循與生產環境相同的步驟。確保已設定生產域名和 API 密鑰。

首先，前往 [Webhooks admin](https://fastcomments.com/auth/my-account/manage-data/webhooks)。此頁面可透過「管理資料」->「Webhooks」存取。

設定頁面如下所示：

[app-screenshot-start url='/auth/my-account/manage-data/webhooks'; selector = '.content'; alt='Webhooks 管理頁面，包含每個評論事件的域名選擇器和端點 URL 欄位，以及「發送測試有效負載」'; title='Webhooks 設定'; cacheBuster = 'v3' app-screenshot-end]

在此頁面中，您可以為每種評論事件指定端點。

對於每種事件，請務必點擊「發送測試有效負載」以確保已正確設定整合。詳情請參閱下一節「Testing」。
---