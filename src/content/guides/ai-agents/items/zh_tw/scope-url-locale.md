預設情況下，agent 在整個租戶執行 —— 每個頁面、每個語系。編輯表單上的 **Scope** 和 **Locales** 區段讓你可以縮小範圍。

### Restrict to specific pages

**Restrict to specific pages** 欄位接受每行一個 URL pattern，採用 url-pattern glob syntax。agent 只會在頁面 URL 符合至少一個模式的留言上執行。範例：

- `/news/*` - `/news` 底下的任何頁面。
- `/forums/*` - `/forums` 底下的任何頁面。
- `/blog/2026/*` - `/blog/2026` 底下的任何頁面。
- (多行一起) - 如果 **任一** 模式符合，agent 就會執行。

最多：每個 agent 50 個模式。模式必須是有效的 url-pattern globs——表單會以特定錯誤拒絕格式錯誤的模式。

當此欄位為 **空白** 時，agent 會在租戶中的每個頁面上執行。

當此欄位為 **非空白** 時，agent 採取封閉失敗（fails closed）：任何留言沒有 `urlId`（例如沒有頁面上下文的租戶層級事件）的觸發都會被跳過。這是設計使然——「限定為 /news/*」不應該悄悄地變成「全部」。

### Restrict to specific locales

**Restrict to specific locales** 的雙欄選擇器接受 FastComments 的 locale ID（`en_us`, `zh_cn`, `de_de` 等）。agent 只會在偵測到的語系位於所選清單內的留言上執行。

偵測到的語系來自留言的 `locale` 欄位，該欄位由留言元件在發佈時根據頁面語系設定。

當**未選取任何語系**時，agent 會在所有語系上執行。

當**選取一個或多個語系**時，agent 採取封閉失敗：沒有留言的觸發，或留言沒有 `locale` 欄位的觸發，會被跳過。

### Combined scoping

URL 與語系篩選是以 AND 的方式共同套用。只有在 **兩者** 都允許時，觸發器才會啟動 agent。

實用範例：
- `/news/*` URL pattern + `en_us` locale - 只有英文的新聞區域。
- 沒有 URL 篩選 + 多個語系 - 在整個租戶有效，但僅限為這個 agent 的提示所撰寫的語言。

### Why scope an agent

- **Cost.** 範圍限制會減少 agent 必須處理的觸發數量，從而降低 token 花費。
- **Correctness.** 為技術性文章調校的摘要提示詞在產品頁面上可能會產生不良輸出。設定範圍比起用英文要求提示詞「跳過非技術性頁面」更為精確。
- **Locale-specific behavior.** 一個只用德文寫歡迎詞的歡迎機器人應該只在德文語系的留言上執行。將 `de_de` 語系範圍與在 [initial prompt](#personality-prompt) 中使用德語語調結合。

### What scoping does *not* do

- 它不會改變 **agent slot count**（參見 [Plans and Eligibility](#plans-and-eligibility)）——被範圍限制的 agent 仍然佔用一個槽位。
- 它不會改變 [Budget caps](#budgets-overview) —— 每個 agent 的每日與每月上限會套用於所有符合條件的觸發。
- 它不會回溯地套用在過去的執行紀錄上——執行記錄會顯示 agent 過去所做的一切，即使你之後把範圍縮小。

---