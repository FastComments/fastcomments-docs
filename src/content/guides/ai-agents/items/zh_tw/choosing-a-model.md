每個 Agent 都會使用兩種 LLM 模型之一執行，可在編輯表單的 **Model** 區段中選擇。

### 兩個選項

- **GLM 5.1 (DeepInfra) - 更聰明，稍慢** - 預設選項。推理品質較高，每次呼叫稍慢。建議用於審核類型的 Agent(`Moderator` 範本，以及任何呼叫 `ban_user` 或 `mark_comment_spam` 的情況)，因為這類情況下錯誤判斷的代價很高。

- **GPT-OSS 120B Turbo (DeepInfra) - 更快** - 每次呼叫更快，延遲更低。建議用於高流量、低風險的 Agent(歡迎訊息、討論串置頂)，這類情況下您希望幾秒內得到回應，且判斷失誤的後果輕微。

兩種模型都支援函式呼叫，都透過相同的 OpenAI 相容 API 執行，並共用相同的每個工具 schema - 因此您可以隨時在已儲存的 Agent 之間切換，無需更改其他設定。

### 成本差異

兩種模型的每 token 成本不同。Agent 的 [預算上限](#budgets-overview) 是以您的帳戶貨幣計價，而非以 token 計價，因此從一種模型切換到另一種會改變您的每日與每月上限內可容納的執行次數。[Run History](#run-history) 頁面會在執行完成後顯示以您的貨幣計算的每次執行成本 - 切換後觀察幾次執行，是評估新消耗率最簡單的方法。

### 每次執行的 Token 數

模型的回應 token 用量會在每次觸發時記錄為 **tokensUsed**。它包含在 `trigger.succeeded` 與 `trigger.failed` 的 webhook 負載中(見 [Webhook Payloads](#webhook-payloads)),並顯示在 [Run Detail View](#run-detail-view) 中。用量取決於：

- 您包含多少 [Context](#context-options) - 討論串內容、使用者歷史記錄與頁面中繼資料都會增加 token。
- 您的 [Initial prompt](#personality-prompt) 與 [Community guidelines](#community-guidelines) 有多長。
- Agent 在單次執行中呼叫多少工具(每次工具呼叫及其結果都會與模型往返)。

**Max Tokens Per Trigger**(預設 20,000)是每次執行的上限，按每個 Agent 個別設定。

### 切換模型

您可以隨時在編輯表單上切換模型。現有的執行歷史與分析會保留其原始的 token 與成本數字 - 這些是在執行當下記錄的。新模型僅適用於您儲存之後開始的執行。

沒有「使用較便宜模型」的選項。選擇是按每個 Agent 明確指定的。