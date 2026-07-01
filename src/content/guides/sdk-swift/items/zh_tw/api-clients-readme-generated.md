FastComments SDK 提供三個 API 客戶端：

### PublicAPI - Client-Safe Methods

`PublicAPI` 包含可安全從客戶端程式碼（iOS/macOS 應用）呼叫的方法。這些方法：
- 不需要 API 金鑰
- 可使用 SSO 令牌進行驗證
- 依使用者/裝置進行速率限制
- 適用於面向最終使用者的應用程式

**範例使用情境**: 在 iOS 應用中取得與建立評論

### DefaultAPI - Server-Side Methods

`DefaultAPI` 包含需要 API 金鑰的已驗證方法。這些方法：
- 需要您的 FastComments API 金鑰
- 僅應從伺服器端程式碼呼叫
- 完全存取您的 FastComments 資料
- 依租戶進行速率限制

**範例使用情境**: 管理操作、大量資料匯出、使用者管理

### ModerationAPI - Moderator Dashboard Methods

`ModerationAPI` 提供完整的即時與快速審核 API 套件。每個 `ModerationAPI` 方法皆接受 `sso` 參數，並可透過 SSO 或 FastComments.com 會話 Cookie 進行驗證。

**範例使用情境**: 為您社群的審核者建立審核體驗

**重要**: 絕不要在客戶端程式碼中暴露您的 API 金鑰。API 金鑰只能在伺服器端使用。