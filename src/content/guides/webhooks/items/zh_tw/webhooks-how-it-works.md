系統中對 Comment 物件的所有變更都會觸發一個事件，該事件最終會進入佇列。

初始的 webhook 事件通常會在事件來源發生後約六秒內發送。

你可以在 Webhooks 管理介面中監控此佇列，以防你的 API 故障。

如果對你的 API 的請求失敗，我們會依排程將其重新放入佇列。

該排程為 `1 Minute * the retry count`。如果呼叫失敗一次，系統將在
一分鐘後重試。如果失敗兩次，則會等待兩分鐘，依此類推。這樣做是為了
避免在你因為負載相關原因而出現故障時，對你的 API 造成過大的負載。

Webhooks 可以從 [記錄頁面](https://fastcomments.com/auth/my-account/manage-data/webhooks/logs) 取消。