#### "註冊令牌未找到、已過期或已被使用"

您註冊 URL 中的令牌 (<a href="https://fastcomments.com/auth/my-account/lti-config" target="_blank">在此取得</a>) 有效期為 30 分鐘且只能使用一次。如果您的 LMS 花了比較久的時間，或是在註冊成功後又重試，該令牌會被拒絕。請在 FastComments LTI 1.3 Configuration 頁面產生新的 URL 並重新開始。

#### "Platform rejected registration"

您的 LMS 拒絕了註冊握手。最常見的原因：

- **Tool already registered with the same client name.** 有些平台（尤其是 D2L）會拒絕再次註冊名稱為 "FastComments" 的工具，除非先刪除先前的設定。請在您的 LMS 中移除舊的工具，然後重試。
- **Wrong field in the LMS.** 確認您是把 URL 貼到 **registration / tool initiation registration endpoint** 欄位，而不是 launch URL 或 login URL 欄位。
- **The LMS doesn't actually support Dynamic Registration.** 舊版的 Moodle 和 Blackboard 會宣稱支援 LTI 1.3，但只允許手動設定。請檢查您平台的文件。

#### "Failed to fetch platform configuration"

FastComments 無法讀取您 LMS 的 openid-configuration 文件。這種情況少見，通常表示 LMS 提供了格式錯誤或無法存取的 discovery URL。請聯絡您的 LMS 支援人員。

#### Launch shows "Configuration not found"

要麼是 FastComments 的設定被刪除，要麼是啟動來自我們不認得的 `iss`/`client_id` 組合。如果您刪除並重新註冊過，請指示 LMS 移除並重新加入 FastComments 工具，以便取得新的 client_id。

#### Launch shows "Deployment not registered"

您是從與最初啟動不同的 Brightspace/Moodle/Blackboard 部署啟動 FastComments。FastComments 會在首次啟動時把 `deployment_id` 鎖定作為安全檢查。要在同一 client 底下新增部署，請聯絡支援 — 我們會把該部署 ID 加到設定中。

#### Launch shows "Unsupported message_type"

LMS 傳送了 FastComments 不處理的 LTI 訊息（例如 `LtiSubmissionReviewRequest`）。FastComments 僅支援標準的 resource-link 啟動與 deep-linking 流程。如需加入特定的訊息類型，請與我們聯絡。

#### Iframe doesn't resize

大多數 LMS 會自動調整 LTI iframe 的大小。如果您的 LMS 沒有這麼做，請確認 LMS 的啟動設定允許工具向父框架傳送 postMessage 事件。FastComments 會發出 Canvas-style（`lti.frameResize`）和 IMS-spec（`org.imsglobal.lti.frameResize`）兩種調整大小的訊息。