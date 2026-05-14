#### "註冊令牌未找到、已過期或已被使用"

註冊 URL 中的令牌有效期為 30 分鐘，且只能使用一次。如果你的 LMS 花費的時間超過此時間，或在註冊已成功後重新嘗試註冊，該令牌將被拒絕。請在 FastComments 的 LTI 1.3 Configuration 頁面中產生新的 URL 並重新開始。

#### "Platform rejected registration"

你的 LMS 拒絕了註冊協商。最常見的原因：

- **工具已用相同的 client name 註冊。** 有些平台（尤其是 D2L）會拒絕第二次註冊「FastComments」，直到先前的註冊被刪除。請在你的 LMS 中移除舊的工具，然後重試。
- **在 LMS 中填錯欄位。** 確認你將 URL 貼到 **registration / tool initiation registration endpoint** 欄位，而不是 launch URL 或 login URL 欄位。
- **該 LMS 實際上不支援 Dynamic Registration。** 較舊的 Moodle 和 Blackboard 版本雖然標示支援 LTI 1.3，但僅允許手動配置。請檢查你平台的文件。

#### "Failed to fetch platform configuration"

FastComments 無法讀取你 LMS 的 openid-configuration 文件。這種情況很少見，通常代表 LMS 提供了格式錯誤或無法存取的 discovery URL。請聯絡你的 LMS 支援人員。

#### Launch shows "Configuration not found"

要麼 FastComments 中的設定被刪除，要麼該啟動來自我們不識別的 `iss`/`client_id` 配對。如果你刪除後重新註冊，請指示你的 LMS 移除並重新加入 FastComments 工具，以便其獲得新的 client_id。

#### Launch shows "Deployment not registered"

你從與最初啟動不同的 Brightspace/Moodle/Blackboard 部署啟動了 FastComments。FastComments 會在第一次啟動時將 `deployment_id` 鎖定作為安全檢查。要在同一個 client 下新增部署，請聯絡支援 - 我們會將該 deployment ID 加到設定中。

#### Launch shows "Unsupported message_type"

LMS 傳送了 FastComments 無法處理的 LTI 訊息（例如 `LtiSubmissionReviewRequest`）。FastComments 只支援標準的 resource-link 啟動和 deep-linking 流程。如果你需要加入特定的訊息類型，請聯絡我們。

#### Iframe doesn't resize

大多數 LMS 會自動調整 LTI iframe 的大小。如果你的 LMS 沒有，請確認 LMS 的啟動設定允許該工具向父框架傳送 postMessage 事件。FastComments 會發出 Canvas-style（`lti.frameResize`）和 IMS-spec（`org.imsglobal.lti.frameResize`）兩種 resize 訊息。