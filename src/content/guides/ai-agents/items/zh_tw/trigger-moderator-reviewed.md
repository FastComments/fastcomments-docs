---
當版主將留言標記為已審查時觸發。

### 代理接收的上下文

- 該留言。
- **triggering user ID** - 進行審查的版主。
- 可選的主題 / 使用者歷史 / 頁面上下文，依設定而定。

### 誰會觸發此事件

由人類版主在審核頁面、留言小工具，或透過 API 執行的操作。

### 常見用途

- **Audit forwarding** 透過 [Webhooks](#webhooks-overview)。
- **Memory writes** - 記錄一則記憶備註，表明此留言已由人類審查，以免其他代理重複處理。

### 注意事項

- "Reviewed" 是 moderation queue states 中與 "approved" 和 "spam" 分別追蹤的狀態之一。留言可以是 approved-and-reviewed、approved-but-not-reviewed 等等。請參見審核指南中的 [審核如何運作](/guide-moderation.html#moderation-approvals)。
- 在擁有大量版主的租戶上，此觸發器的頻率會很高。請有選擇地訂閱並相應規劃預算。

---