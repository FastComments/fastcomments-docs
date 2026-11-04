#### 「註冊權杖未找到、已過期或已被使用」

註冊 URL 中的權杖（<a href="https://fastcomments.com/auth/my-account/lti-config" target="_blank">在此取得</a>）有效期為 30 分鐘，且只能使用一次。如果您的 LMS 花費的時間超過這個限制，或者註冊在成功後被重試，該權杖將被拒絕。請在 FastComments LTI 1.3 設定頁面產生一個新的 URL 並重新開始。

#### 「平台拒絕了註冊」

您的 LMS 拒絕了註冊握手。最常見的原因：

- **工具已以相同的用戶端名稱註冊。** 某些平台（特別是 D2L）會拒絕第二次註冊「FastComments」，直到先前的註冊被刪除。請在您的 LMS 中移除舊的工具，然後重試。
- **在 LMS 中填錯了欄位。** 請確保您將 URL 貼到 **registration / tool initiation registration endpoint** 欄位，而不是啟動 URL 或登入 URL 欄位。
- **LMS 實際上不支援動態註冊。** 較舊的 Moodle 和 Blackboard 版本宣稱支援 LTI 1.3，但只允許手動設定。請查看您平台的文件。

#### 「無法取得平台設定」

FastComments 無法讀取您 LMS 的 openid-configuration 文件。這種情況很少見，通常表示 LMS 提供了格式錯誤或無法存取的探索 URL。請聯絡您的 LMS 支援團隊。

#### 啟動時顯示「找不到設定」

可能是 FastComments 中的設定已被刪除，或者啟動請求來自我們無法識別的 `iss`/`client_id` 配對。如果您刪除並重新註冊了，請指示您的 LMS 移除並重新新增 FastComments 工具，以便它取得新的 client_id。

#### 啟動時顯示「部署未註冊」

您是從與首次啟動時不同的 Brightspace/Moodle/Blackboard 部署啟動 FastComments 的。FastComments 會在首次啟動時固定 `deployment_id` 作為安全檢查。若要在同一用戶端下新增新的部署，請聯絡支援團隊——我們會將部署 ID 新增到設定中。

#### 啟動時顯示「不支援的 message_type」

LMS 傳送了 FastComments 不處理的 LTI 訊息（例如 `LtiSubmissionReviewRequest`）。FastComments 僅支援標準的資源連結啟動和深層連結流程。如果您需要新增特定的訊息類型，請與我們聯絡。

#### Iframe 無法調整大小

大多數 LMS 會自動調整 LTI iframe 的大小。如果您的 LMS 不會，請檢查 LMS 的啟動設定是否允許工具向父框架傳送 postMessage 事件。FastComments 會同時發送 Canvas 風格（`lti.frameResize`）和 IMS 規範（`org.imsglobal.lti.frameResize`）的調整大小訊息。