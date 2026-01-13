對於 `localhost`，請按照與 production 相同的步驟操作。請確保您已設定 production 網域與 API Secrets。

首先，前往 [Webhooks 管理](https://fastcomments.com/auth/my-account/manage-data/webhooks)。可透過 Manage Data -> Webhooks 存取。

The configuration page appears as follows:

[app-screenshot-start url='/auth/my-account/manage-data/webhooks'; selector = '.content'; title='Webhooks Configuration'; cacheBuster = 'v3' app-screenshot-end]

在此頁面中，您可以為每種類型的留言事件指定端點。

對於每種事件類型，請務必點擊 Send Test Payload 以確認您的整合設定正確。詳情請參見下一節「Testing」。

---