Agent 成本是以 **token 為基準**。每次 LLM 呼叫都會回傳 token 數量，平台會依據該模型的每 token 費率將其換算為美元美分，而這些美分會計入 agent 與租戶的預算中。

### 計費項目

- **所有 LLM 呼叫**，包括產生零個工具動作的呼叫（「agent 決定不做任何事」）。即使沒有產生任何動作，推論費用仍需支付。
- **Dry-run 呼叫**。Dry-run 是「不執行動作，但仍呼叫 LLM」— LLM 呼叫的成本相同。請參閱 [Dry-Run 模式](#dry-run-mode)。
- **Replay 呼叫**。Replay 是針對歷史留言執行的 dry-run。它們會消耗 token。請參閱 [測試執行（Replay）](#test-runs-replays)。

### 不計費項目

- **從未產生 LLM 呼叫的觸發。** 在 LLM 之前就被丟棄的情況（超出預算、被速率限制、範圍不符、計費無效、迴圈防護）不消耗任何 token。請參閱 [丟棄原因](#drop-reasons)。
- **工具分派。** 呼叫 `pin_comment` 或任何其他工具本身不消耗 token — 只有 LLM 的來回往返才會。
- **`search_memory`。** 它是唯讀的，不會產生自己的 LLM 來回往返。

### 每次執行的成本

單次 agent 執行可以多次呼叫 LLM — 每個工具呼叫的結果都會回饋給模型，讓它可以再呼叫另一個工具或結束。因此，一次執行的 `tokensUsed` 是該次執行中所有 LLM 來回往返的總和。

每次執行 token 成本的最大貢獻因素：

- **冗長的[初始提示詞](#personality-prompt)與[社群守則](#community-guidelines)** — 每次執行都會包含它們。
- **[內容選項](#context-options)** — 討論串內容、使用者歷史、頁面中繼資料。每一項都會增加 token。
- **留言文字本身** — 冗長的留言成本更高。
- **一次執行中的多次工具呼叫** — 每個工具的結果訊息都會送回模型。
- **記憶讀取** — `search_memory` 最多回傳 25 筆記錄（總內容上限 8000 字元）。其中大部分位元組會進入下一個提示詞。

**每次觸發的最大 Token 數**（預設 20,000）限制的是每次 LLM 呼叫的**回應**大小。它不會限制輸入大小。

### Token 換算為美分

平台套用單一的每租戶方案費率（每 `flexLLMUnit` token 對應 `flexLLMCostCents` 美分）。每 token 成本是方案層級的，而非按模型區分 — 在特定方案上，兩個可用模型（[GLM 5.1 與 GPT-OSS Turbo](#choosing-a-model)）都以相同費率計費。[執行詳情檢視](#run-detail-view)會在執行完成後以您的貨幣顯示每次執行的成本。

### 成本記錄位置

每次執行都會記錄其原始 token 數量與每次執行的成本。每日與每月總計會彙總至 [Analytics 頁面](#analytics-page)。

### 如何查看成本

- **每次執行的成本**：[執行詳情檢視](#run-detail-view) -> `Cost` 欄位。
- **每日 / 每月彙總**：[Analytics 頁面](#analytics-page) -> 預算使用量與每日成本圖表。
- **每次動作的成本**：同樣在執行詳情檢視中，當 agent 的工具迴圈異常冗長時，對調整很有用。

### 另請參閱

- [選擇模型](#choosing-a-model) — 對成本影響更大的槓桿。
- [內容選項](#context-options) — 額外成本的來源。
- [預算總覽](#budgets-overview) — 防止成本失控的硬性上限。