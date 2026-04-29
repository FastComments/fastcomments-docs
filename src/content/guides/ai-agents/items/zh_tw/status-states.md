代理有三種狀態之一：

### Disabled

代理已關閉。 不會處理任何觸發器，且代理不會出現在派送路徑中。其執行歷史、分析與記憶仍然保留 —— 若您之後重新啟用它，歷史資料仍在。

在以下情況使用 `Disabled`：
- 您想將代理從輪替中移出但不想刪除它。
- 代理出現異常行為，需要立即停止以便調查。
- 您以季節性方式輪替代理（例如只在假期啟用的迎賓代理）。

### Dry Run - default for new agents

代理會端到端執行 —— 處理觸發器、呼叫 LLM、選擇工具呼叫、計算論證與信心度 —— 但 **不會採取任何實際動作**。每次執行都會在 [執行紀錄](#run-history) 中以 **Dry Run** 徽章記錄。

在以下情況使用 `Dry Run`：
- 新的代理剛建立並啟用。每個入門範本預設為 dry-run。
- 您編輯了 prompt 或更改了觸發器集，想在正式套用前查看變更效果。
- 您正在執行[測試執行 / 重播](#test-runs-replays)（重播會強制為 dry-run，無論代理狀態為何）。

平台仍會對 dry-run 執行收取 token —— LLM 的呼叫仍會發生，僅省略副作用。預算上限也適用於 dry-run。參見 [預算概覽](#budgets-overview)。

### Enabled

代理會採取實際動作。工具呼叫會執行 —— 或在該動作受限時排入[審核](#approval-workflow)佇列。

在 dry-run 的輸出看起來正確後使用 `Enabled`。

### Switching status

您可以在編輯表單上在任兩種狀態間切換。從 Dry Run 切換到 Enabled 不會回溯重新執行先前的 dry-run 動作 —— 那些仍會保留為 dry-run 歷史。從該時刻起的新觸發器會開始實際執行。

在執行中途從 Enabled 切換到 Disabled 並**不**會中止正在進行的執行。當前正在執行的觸發器會完成（不論它已經開始了什麼）；下一個觸發器會被丟棄，因為代理現在是 Disabled。

### Status during billing problems

如果您租戶的帳單失效，所有代理皆會被暫停（不論儲存的狀態為何）——觸發器會被丟棄並回傳 `BILLING_INVALID`，直到帳單恢復。儲存的狀態欄位不會改變；dispatcher 只是拒絕執行。參見 [方案與資格](#plans-and-eligibility)。