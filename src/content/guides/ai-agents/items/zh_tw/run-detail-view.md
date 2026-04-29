點選 [Run History](#run-history) 中某一列的 **View** 會開啟該次執行的詳細頁面。在這裡你可以閱讀 agent 的推理並評斷其決策。

### Top: run summary

- **Agent** - 執行該次任務的 agent。
- **When** - 時間戳記。
- **Status** - 已開始 / 成功 / 錯誤，若適用會顯示 **Dry Run** 標章。
- **Cost** - 以你租戶貨幣計算的單次執行成本。
- **Cost per action** - 成本除以非待定動作數量，有助於發現異常高成本的執行。

### Actions taken

列出該次執行所做的每一個工具呼叫，依序排列。每個項目顯示：

- **Action label** - 「Wrote a comment」、「Marked a comment as spam」、「Banned a user」等。該標籤對應於 action type 的 enum。
- **Reference ID** - 受影響的評論、使用者或徽章 ID，以等寬字體顯示（非超連結）。
- **Agent reasoning** - agent 在呼叫時提供的理由。
- **Confidence** - agent 自評的信心值，以百分比顯示。
- **Pending approval** 標章 - 若該動作被排入 [approvals inbox](#approval-workflow) 等待核准而非即時執行，則顯示此標章。

如果該次執行未採取任何動作，本區會顯示：「No actions were taken during this run.」

### LLM transcript

在 Actions 之下，會看到 agent 與 LLM 之間對話的完整逐字稿：

- **System** - 系統提示（platform suffix + 你的初始提示 + 社群守則）。
- **User** - 描述觸發情境的上下文訊息。
- **Assistant** - 模型的回應，包括工具呼叫。
- **Tool** - 回饋給模型的工具結果（例如 `search_memory` 回傳的內容）。

較長的訊息可摺疊；點選 **Expand** / **Collapse** 以檢視。

### Reading transcripts

逐字稿是調整設定時最重要的頁面。當 agent 做出你不同意的決定時，回頭閱讀逐字稿以了解：

- 模型「看到了」什麼（User 的上下文訊息）。
- 模型「決定了」什麼（Assistant 的工具呼叫）。
- 模型「考慮了」什麼（任何工具結果 — 例如 agent 是否真的呼叫了 `search_memory`，以及在封鎖前是否有找到任何內容）。

如果模型持續犯相同類型的錯誤，請編輯 [initial prompt](#personality-prompt) — 或從被拒絕的核准中使用 [Refining Prompts](#refining-prompts)。

### Action references

參考 ID 以等寬字體顯示（非超連結）：

- Comments：評論 ID。
- Users：使用者 ID。
- Badges：徽章 ID。

你可以複製該 ID，然後到相關的審核/管理頁面查詢受影響的記錄。

### What's missing in dry-run

Dry-run 的執行會顯示 **相同** 的動作、理由與信心分數。唯一的差別是在狀態列上會看到 **Dry Run** 標章。評論 / 使用者 / 徽章 的參考 ID 仍會顯示 — 只是 agent 並未實際影響它們。

### Errors

對於處於 `Error` 狀態的執行，詳細頁會顯示底層錯誤訊息。常見錯誤包括：

- **No LLM API key configured** - 租戶或平台設定錯誤。
- **LLM call timeout** - LLM 供應商回應緩慢或無法使用。
- **Tool dispatch failure** - agent 選擇了參數有誤的工具（例如不存在的評論 ID）。
- **Budget exhausted mid-run** - 執行期間達到代理的預算上限，執行被中止。

錯誤不會回溯已完成的部分動作 — 在錯誤發生前已完成的任何工具呼叫仍然有效。