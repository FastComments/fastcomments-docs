FastComments SDK 提供兩種類型的 API 端點：

### PublicAPI - 客戶端安全端點

The `PublicAPI` 包含可安全從客戶端程式碼（iOS/macOS 應用程式）呼叫的端點。這些端點：
- 不需要 API 金鑰
- 可以使用 SSO tokens 進行驗證
- 對每個使用者/裝置實施速率限制
- 適用於面向終端使用者的應用程式

**範例用途**: 在您的 iOS 應用程式中擷取和建立評論

### DefaultAPI - 伺服器端端點

The `DefaultAPI` 包含需要 API 金鑰的已驗證端點。這些端點：
- 需要您的 FastComments API 金鑰
- 僅應從伺服器端程式碼呼叫
- 提供對您的 FastComments 資料的完整存取
- 以租戶為單位實施速率限制

**範例用途**: 管理操作、批次資料匯出、審核工具

**重要**: 絕不要在客戶端程式碼中暴露您的 API 金鑰。API 金鑰應僅用於伺服器端。