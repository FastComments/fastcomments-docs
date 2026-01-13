### Real-Time Updates

Collab Chat 使用 WebSocket 連線即時同步所有已連線使用者之間的對話。當有人建立新的註解、新增評論或刪除討論時，所有正在檢視同一頁面的其他使用者會立即看到更新，無需重新整理。

### How WebSocket Sync Works

當您初始化 Collab Chat 時，小工具會與 FastComments 伺服器建立 WebSocket 連線。此連線在使用者的會話期間保持開啟，並監聽與當前頁面相關的更新。

WebSocket 系統對 Collab Chat 使用三種廣播訊息類型。`new-text-chat` 事件在有人於頁面上建立新註解時觸發。`updated-text-chat` 事件在有人更新既有對話時觸發。`deleted-text-chat` 事件在有人刪除註解時觸發。

### Broadcast ID System

為了避免出現使用者看到自己動作回播的回音效應，每次更新都包含唯一的 `broadcastId`。當使用者建立或更新註解時，他們的用戶端會為該操作產生一個 UUID。當 WebSocket 將更新廣播回所有用戶端時，來源用戶端會忽略該更新，因為它與自身的 `broadcastId` 相符。

這能確保使用者在 UI 中立即看到自己的變更，而無需等待經由伺服器的往返，同時也能確保所有其他使用者接收到該更新。

### Live User Count

頂部列會顯示目前正在檢視該頁面的使用者數量。此計數會在使用者加入與離開時即時更新。使用者數量是透過相同的 WebSocket 連線提供，並根據連線與斷線事件自動遞增/遞減。

### Connection Resilience

如果 WebSocket 連線因網路問題或伺服器維護而中斷，小工具會自動嘗試重新連線。在重新連線期間，使用者仍可與現有註解互動，但在連線重新建立之前，他們將無法看到其他使用者的即時更新。

一旦重新連線，小工具會重新同步以確保沒有遺漏任何更新。此過程會透明進行，無需使用者介入。

### Bandwidth Considerations

WebSocket 訊息為輕量級，只包含同步狀態所需的必要資訊。建立新註解通常使用少於 1KB 的頻寬。系統也包含智慧型分批機制，以在高活動期間降低訊息頻率。

您在 FastComments 儀表板中的使用量指標會追蹤 `pubSubMessageCount` 與 `pubSubBandwidth`，以便您監控跨網站的即時同步活動。

### Cross-Tab Synchronization

如果使用者在多個瀏覽器分頁開啟同一頁面，某一分頁的更新會立即出現在其他分頁。這透過相同的 WebSocket 同步機制運作，且不需要額外設定。

### Security

WebSocket 訊息透過安全連線 (WSS) 傳輸，並包含租戶驗證以確保使用者僅接收其有權查看的對話更新。伺服器會在廣播之前驗證所有操作，以防止未經授權的存取或篡改。