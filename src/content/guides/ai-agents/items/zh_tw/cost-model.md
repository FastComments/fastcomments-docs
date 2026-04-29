Agent cost is **token-based**。每次 LLM 呼叫都會回傳一個 token 計數，平台會使用模型的每 token 費率將其轉換為美分（USD cents），並將這些美分計入代理人與租戶的預算。

### What's billed

- **所有 LLM 呼叫**，包括產生零工具動作的呼叫（「代理人決定不採取任何動作」）。即使沒有產生任何動作，推理也需付費。
- **Dry-run 呼叫**。Dry-run 是「不採取行動，但仍呼叫 LLM」——LLM 呼叫成本相同。參見 [Dry-Run Mode](#dry-run-mode)。
- **Replay 呼叫**。Replay 是對歷史留言進行的 dry-run。它們也會產生 token 成本。參見 [Test Runs (Replays)](#test-runs-replays)。

### What's not billed

- **從未產生 LLM 呼叫的觸發。** 在 LLM 之前被丟棄的情況（超出預算、被速率限制、範圍不符、計費無效、避免迴圈）不會產生任何 token 成本。參見 [Drop Reasons](#drop-reasons)。
- **工具派發。** 呼叫 `pin_comment` 或其他任何工具本身不會產生 token 成本——只有 LLM 的往返才會。
- **`search_memory`。** 它是唯讀的，且不會產生自己的 LLM 往返。

### Cost per run

單次代理人執行（run）可能會多次呼叫 LLM——每次工具呼叫的結果都會回饋給模型，讓模型可以繼續呼叫其他工具或結束。因此一個 run 的 `tokensUsed` 是該 run 中所有 LLM 往返的總和。

單次執行的 token 成本最大貢獻者：

- **冗長的 [initial prompts](#personality-prompt) 與 [community guidelines](#community-guidelines)** — 它們會在每次執行時加入。
- **[Context options](#context-options)** — 討論串上下文、使用者歷史、頁面 metadata。每一項都會增加 token 數。
- **留言內容本身** — 留言越長成本越高。
- **一次執行中多次工具呼叫** — 每個工具的結果訊息都會被送回模型。
- **記憶讀取** — `search_memory` 最多回傳 25 筆紀錄（內容總長上限 8000 個字元）。大部分那些位元會進入下一個提示中。

**Max Tokens Per Trigger**（預設 20,000）限制每次 LLM 呼叫的**回應**大小。它不限制輸入大小。

### Token-to-cents conversion

平台套用單一的每租戶套裝費率（每 `flexLLMUnit` 個 token 為 `flexLLMCostCents` 美分）。每 token 成本是套裝等級的，而非依模型而定——在同一套裝下可用的模型（[GLM 5.1 and GPT-OSS Turbo](#choosing-a-model)）都以相同費率計費。當執行完成後，[Run Detail View](#run-detail-view) 會以您的貨幣顯示該次執行的費用。

### Where cost is recorded

每次執行都會記錄其原始 token 數與每次執行的費用。每日與每月總額會彙整到 [Analytics page](#analytics-page)。

### How to read cost

- **單次執行費用**：至 [Run Detail View](#run-detail-view) -> `Cost` 欄位。
- **每日 / 每月合計**：至 [Analytics page](#analytics-page) -> 預算使用與每日成本圖表。
- **每個動作的成本**：也會顯示在 Run Detail View，當代理人的工具迴圈異常冗長時，此資料有助於調整。

### See also

- [Choosing a Model](#choosing-a-model) - 對成本影響最大的調整項。
- [Context Options](#context-options) - 顯示額外成本來源的位置。
- [Budgets Overview](#budgets-overview) - 防止成本失控的硬性上限。