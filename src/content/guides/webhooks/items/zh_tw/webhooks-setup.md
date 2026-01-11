針對 `localhost`，請採用與正式環境相同的步驟。確保您已設定正式環境的網域與 API Secrets。

首先，前往 [Webhooks admin](https://fastcomments.com/auth/my-account/manage-data/webhooks)。可由 Manage Data -> Webhooks 存取。

設定頁面如下：

[app-screenshot-start url='/auth/my-account/manage-data/webhooks'; selector = '.content'; title='Webhooks Configuration' app-screenshot-end]

在此頁面中，您可以為每種類型的評論事件指定端點。

對於每一類事件，請務必點選 傳送測試載荷 以確保您已正確設定整合。詳細資料請參閱下一節「測試」。