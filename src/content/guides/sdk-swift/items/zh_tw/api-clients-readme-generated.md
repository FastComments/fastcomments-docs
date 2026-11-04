FastComments SDK 提供三種 API 用戶端：

### PublicAPI - 用戶端安全的方法

`PublicAPI` 包含可安全從用戶端程式碼（iOS/macOS 應用程式）呼叫的方法。這些方法：
- 不需要 API 金鑰
- 可使用 SSO 權杖進行驗證
- 依使用者/裝置進行速率限制
- 適合面向終端使用者的應用程式

**範例使用情境**：在您的 iOS 應用程式中取得與建立留言

### DefaultAPI - 伺服器端方法

`DefaultAPI` 包含需要 API 金鑰的已驗證方法。這些方法：
- 需要您的 FastComments API 金鑰
- 只應從伺服器端程式碼呼叫
- 提供對您 FastComments 資料的完整存取權限
- 依租戶進行速率限制

**範例使用情境**：管理操作、大量資料匯出、使用者管理

### ModerationAPI - 審核儀表板方法

`ModerationAPI` 提供一套廣泛的即時且快速的審核 API。每個 `ModerationAPI` 方法都接受 `sso` 參數，並可透過 SSO 或 FastComments.com 的 session cookie 進行驗證。

**範例使用情境**：為您社群的審核員建立審核體驗

**重要事項**：切勿在用戶端程式碼中公開您的 API 金鑰。API 金鑰應僅在伺服器端使用。