在 [Run History](#run-history) 中點擊某一列的 **View** 會開啟單次執行的詳細資訊頁面。您可以在這裡閱讀代理的推理過程並判斷其決策。

### 頂部：執行摘要

- **Agent** - 執行的是哪個代理。
- **When** - 時間戳。
- **Status** - Started / Success / Error，若適用還會顯示 **Dry Run** 徽章。
- **Cost** - 以您的租戶貨幣計算的單次執行成本。
- **Cost per action** - 成本除以非 pending 狀態的動作數量，有助於發現異常昂貴的執行。

### 採取的動作

按順序列出該次執行所做的每個工具呼叫。每個項目顯示：

- **Action label** - 「Wrote a comment」、「Marked a comment as spam」、「Banned a user」等等。標籤由動作類型列舉值對應而來。
- **Reference ID** - 受影響的留言、使用者或徽章 ID，以等寬文字顯示（非超連結）。
- **Agent reasoning** - 代理在呼叫時提供的理由。
- **Confidence** - 代理自評的信心程度，以百分比顯示。
- **Pending approval** 徽章 - 若動作被排入 [approvals inbox](#approval-workflow) 而非直接執行。

如果該次執行沒有採取任何動作，此區段會顯示：「No actions were taken during this run.」

### LLM 對話記錄

在動作下方，是代理與 LLM 對話的完整記錄：

- **System** - 系統提示詞（平台後綴 + 您的初始提示詞 + 社群規範）。
- **User** - 描述觸發原因的上下文訊息。
- **Assistant** - 模型的回應，包括工具呼叫。
- **Tool** - 回饋給模型的工具結果（例如 `search_memory` 回傳的內容）。

長訊息可摺疊；點擊 **Expand** / **Collapse** 即可檢視。

### 閱讀對話記錄

對話記錄是調校時最重要的頁面。當代理做出您不同意的決策時，請回頭閱讀以了解：

- 模型**看到**了什麼（User 上下文訊息）。
- 模型**決定**了什麼（Assistant 的工具呼叫）。
- 模型**考量**了什麼（任何工具結果 - 例如，代理是否真的呼叫了 `search_memory`，以及在封禁之前是否找到了任何內容）。

如果模型持續犯同一類錯誤，請編輯[初始提示詞](#personality-prompt) - 或從被拒絕的核准中使用 [Refining Prompts](#refining-prompts)。

### 動作參考

參考 ID 以等寬文字顯示（非超連結）：

- 留言：留言 ID。
- 使用者：使用者 ID。
- 徽章：徽章 ID。

您可以複製該 ID，在相關的審核/管理頁面中查詢受影響的記錄。

### Dry-run 中缺少的內容

Dry-run 執行會顯示**相同**的動作、理由和信心分數。唯一的差異是狀態列上的 **Dry Run** 徽章。留言 / 使用者 / 徽章的參考 ID 仍會顯示 - 代理只是沒有實際影響它們。

### 錯誤

對於處於 `Error` 狀態的執行，詳細資訊頁面會顯示底層的錯誤訊息。常見錯誤：

- **No LLM API key configured** - 租戶或平台設定錯誤。
- **LLM call timeout** - LLM 供應商速度緩慢或無法使用。
- **Tool dispatch failure** - 代理選擇了帶有錯誤參數的工具（例如，一個已不存在的留言 ID）。
- **Budget exhausted mid-run** - 執行進行中時觸及了代理的上限。執行已被中止。

錯誤不會回復部分已執行的動作 - 在錯誤發生前完成的任何工具呼叫都會保留。