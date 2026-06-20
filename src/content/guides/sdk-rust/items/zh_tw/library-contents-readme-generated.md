FastComments 的 Rust SDK 由幾個模組組成：

- **用戶端模組** - FastComments REST API 的 API 用戶端
  - 為所有 API 模型提供完整的類型定義
  - 三個涵蓋所有 FastComments 方法的 API 客戶端：
    - `default_api` (**DefaultApi**) - 適用於伺服器端使用的以 API 金鑰驗證的方法
    - `public_api` (**PublicApi**) - 公開的、不需 API 金鑰的方法，適合從瀏覽器與行動應用程式呼叫
    - `moderation_api` (**ModerationApi**) - 支援管理員儀表板的各種方法，包括留言審核（列出、計數、搜尋、日誌、匯出）、審核操作（移除/還原、檢舉、設定審查/垃圾訊息/核准狀態、投票、重新開啟/關閉討論串）、封鎖（從留言封鎖、撤銷、封鎖前摘要、封鎖狀態/偏好、被封鎖使用者計數）、以及徽章與信任（授予/移除徽章、手動徽章、取得/設定信任因子、使用者內部檔案）。每個 Moderation 方法都接受一個 `sso` 參數，讓呼叫可代表以 SSO 驗證的管理員執行。
  - 完整支援 async/await 與 tokio
  - 詳細的 API 文件請參閱 [client/README.md](https://github.com/FastComments/fastcomments-rust/blob/main/client/README.md)

- **SSO 模組** - 伺服器端的單一登入（Single Sign-On）工具
  - 用於使用者驗證的安全令牌產生
  - 支援簡易與安全兩種 SSO 模式
  - 基於 HMAC-SHA256 的令牌簽名

- **核心類型** - 共用的類型定義與工具
  - 留言模型與後設資料結構
  - 使用者與租戶設定
  - 常見操作的輔助函式