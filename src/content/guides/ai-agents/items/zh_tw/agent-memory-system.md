Agent memory 是一個以租戶為範圍的、**共享** 鍵值池，租戶中的每個 agent 都可以讀取與寫入。它的存在是為了讓 agents 能夠在多次執行間攜帶上下文。

### 為何需要 memory

LLM 的上下文是每次執行獨立的。沒有 memory，當一個 agent 對使用者發出警告時，下一次看到相同使用者時就無法得知之前的警告。平台的升級政策 —「先警告再封鎖」— 依賴於 agent 能找到先前的警告。memory 就是讓這個流程可行的關鍵。

### 兩種 memory

- **WARNING** - 在 [`warn_user`](#tool-warn-user) 流程中自動寫入。agent 不會手動寫入 `WARNING` 記錄；它們是警告使用者時的副作用。
- **NOTE** - 由 [`save_memory`](#tools-overview) 寫入。agent 想讓未來的 agents 知道的一般用途上下文。

在判斷是否應該封鎖時，升級政策會特別查找 `WARNING` 記錄。

### 租戶範圍，共享給 agent

您租戶中的所有 agents 共用 **單一記憶池**。Agent A 存下的 note 對 Agent B 的 `search_memory` 呼叫是可見的。這是刻意如此設計的 — 您希望分流（triage）agent 的註記能影響審核（moderator）agent 的決策。

`tenantId` 由執行者從 agent 所屬的租戶設置 — 永遠不會從 LLM 的引數中設定，因此從設計上不可能發生跨租戶的記憶洩漏。

### 記憶記錄包含哪些內容

每個記憶條目包含：

- **是由哪個 agent 撰寫的，以及時間。**
- **關於誰** — 此記憶描述的使用者。agent 無法偽造；平台會自動從觸發 agent 的來源填入。
- **一個隱藏的替代帳號訊號** — 平台也會（私下）記錄來源評論的 IP 指紋，未來的記憶搜尋可以顯示出來自相同 IP 的其他帳號的註記。該指紋永遠不會顯示給 agent 或 LLM。
- **註記本體** — 最多 2000 字元的自由文本。
- **檢索用的標籤** — 最多 10 個短標籤。
- **種類** — 要麼是 warning 要麼是 general note。
- **一個可選的評論連結** — 如果記憶綁定於特定評論。

### 搜尋行為

[`search_memory`](#tools-overview) 回傳最多 25 筆記錄，排序為由新到舊，自動範圍限制為（觸發者的使用者）或（觸發者 IP 上的其他帳號）。結果的總文字數亦被限制為最多 8000 字元 — 若超出上限，較舊的條目會被丟棄。

agent 不會傳遞 `userId` 或 `targetIpHash`。兩者皆由執行者設定。

### 永久保存

Memory 沒有 **TTL**。記錄會持續存在，直到被明確移除。關於使用者的 WARNING 記錄刻意不會自動刪除 — 若找不到歷史升級記錄，平台的「在封鎖前搜尋」檢查就會失去意義。

移除 memory 的三種方式：

- 管理員刪除基礎評論 — 任何綁定於該評論的記憶會被級聯刪除。
- 使用者被刪除 — 關於該使用者的所有記憶條目會在同一交易中移除。
- 您的租戶被刪除。

目前沒有針對單一記憶記錄的管理介面。

### Dry-run 中的 memory

Dry-run agents **不會**寫入 memory。這是刻意設計：dry-run agent 的假設決策不應該污染共享的記憶池。透過 `search_memory` 的讀取在 dry-run 中仍然正常 — agent 可以看到實際運作中 agents 的記憶 — 只是無法新增記錄。

### Replays 中的 memory

與 dry-run 相同：replay agents 不會寫入 memory。replay 僅供預覽。參見 [Test Runs (Replays)](#test-runs-replays)。

### 限制摘要

| 限制 | 值 |
|---|---|
| Memory 內容最大長度 | 2000 chars |
| Memory 標籤最大長度 | 64 chars |
| Memory 標籤最大數量 | 10 |
| Memory 查詢最大長度 | 200 chars |
| Memory 搜尋結果限制 | 25 records |
| Memory 搜尋總內容上限 | 8000 chars |

### 參見

- [Tool: save_memory](#tools-overview) 用於寫入。
- [Tool: search_memory](#tools-overview) 用於讀取。
- [Tool: warn_user](#tool-warn-user) - 唯一會寫入 WARNING 類型 memory 的工具。
- [Tool: ban_user](#tool-ban-user) - 系統提示要求在此之前呼叫 `search_memory`。