---
FastComments 僅支援對 Comment 資源的 webhook。

我們支援對評論的建立、刪除以及更新的 webhook。

這些在我們系統中被視為獨立的事件，因而具有不同的語意
並且有不同的 webhook 事件結構。

任意數量的端點都可以從儀表板或透過 API 訂閱相同的事件
（請參閱 透過 API 管理 Webhooks）。每個 webhook 都會獨立傳送。

---