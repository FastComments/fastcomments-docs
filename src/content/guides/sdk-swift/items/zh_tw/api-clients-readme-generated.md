FastComments SDK 提供三個 API 用戶端：

### PublicAPI - 可安全於客戶端呼叫的方法

The `PublicAPI` contains methods that are safe to call from client-side code (iOS/macOS apps). These methods:
- 不需要 API key
- 可以使用 SSO 令牌進行驗證
- 對每位使用者/裝置有速率限制
- 適用於面向最終使用者的應用程式

**Example use case**: 在你的 iOS 應用中擷取與建立留言

### DefaultAPI - 伺服器端的方法

The `DefaultAPI` contains authenticated methods that require an API key. These methods:
- 需要你的 FastComments API key
- 應僅在伺服器端程式碼中呼叫
- 提供對你的 FastComments 資料的完整存取
- 對每個 tenant 有速率限制

**Example use case**: 管理操作、大量資料匯出、使用者管理

### ModerationAPI - 版主儀表板的方法

The `ModerationAPI` contains methods that power the moderator dashboard. These methods cover:
- **Comment moderation** - 列出、計數、搜尋、擷取日誌，以及匯出留言
- **Moderation actions** - 移除/還原留言、標記、設定審查/垃圾訊息/核准狀態、管理投票，以及重新開啟/關閉討論串
- **Bans** - 封鎖使用者於留言、還原封鎖、取得封鎖前摘要、檢查封鎖狀態與偏好，並讀取被封鎖使用者數量
- **Badges & trust** - 頒發/移除徽章、列出手動徽章、取得/設定使用者的信任係數，及讀取使用者的內部檔案

Every `ModerationAPI` method accepts an `sso` parameter so moderators can be authenticated via SSO.

**Example use case**: 為你的社群版主建構審核體驗

**IMPORTANT**: Never expose your API key in client-side code. API keys should only be used server-side.