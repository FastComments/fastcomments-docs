警示電子郵件會在 agent 的花費超過其上限的可配置百分比時觸發。它們會寄給負責帳單的人。

### How alerts work

每個 agent 的編輯表單上都有一個 **Alert thresholds** 欄位。預設為 `80%` 和 `100%`。你可以勾選或取消勾選個別的閾值，並且可以新增其他百分比。

當 agent 在特定範圍內（每日或每月）的花費在該期間首次跨越某個閾值時，平台會對每位收件者寄出一封電子郵件。在同一期間內再次跨越該閾值（例如，花費降到 80% 以下又回升）不會重新寄送。

這是以期間為單位：新的每日重置會為當天重新啟動閾值跨越的判定邏輯。

### Tenant-scope alerts

租戶（帳戶）有其自己的每日和每月上限。Tenant-scope alerts 在固定閾值（`80%` 和 `100%`）觸發。這些無法為單一 agent 配置，因為它們適用於整個租戶。

### Recipients

預算警示會寄給：

- 每位在租戶中被標記為 **Super admin** 的使用者。
- 每位在租戶中被標記為 **Billing Admin** 的使用者。

這包括兩個角色的聯集——同時擁有兩者角色的使用者只會收到一封郵件。

### Why both roles

Super admins 通常是需要知道某個 agent 已接近上限的操作人員。Billing admins 擁有發票，無論是否日常管理 agent，都需要知道費用激增的情況。若要實際編輯 agent（提高上限、暫停它），收件者還需要具有 **Customization Admin** 角色——該角色控管 agent 的編輯頁面。

### Per-user opt-out

在個人檔案中選擇退出管理通知的收件者會被跳過。這是控制其他管理通知的相同退出開關。

如果 **all** 收件者都已選擇退出，該警示會被記錄（警告等級），且不會寄出任何電子郵件。

### Email content

電子郵件內容包含：

- **agent display name** 與內部名稱。
- 已跨越的 **scope**（例如 "agent daily budget", "agent monthly budget", "account daily budget", "account monthly budget"）。
- 被跨越的 **threshold percentage**。
- 以租戶貨幣表示的 **Usage**。
- 以租戶貨幣表示的 **Cap**。
- 一個 **one-click signed login link**，點擊後可直接帶收件者前往：
  - The agent edit page，針對 agent 範圍的警示。
  - The AI Agents list page，針對租戶範圍的警示。

該連結已預先驗證，收件者只需一鍵即可提高上限或停用 agent。

### How thresholds fire

平台會追蹤在本期間哪些閾值已經觸發，對 agent 與租戶分別記錄。因此：

- 在同一期間內先跨越 80% 再跨越 100% 會按順序觸發兩者。
- 若一次性從 0% 直接跳到 100%，會觸發最高被跨越的閾值（100%），而非 80%，因此會寄出最嚴重的警示。

### When you stop getting alerts

如果 agent 的花費在本期間從未達到下一個閾值，你在本期間不會收到更多電子郵件。下一次的每日重置（或每月重置）會清除這些追蹤紀錄。

### Disabling alerts

取消勾選你不想要的閾值。如果你不希望對某個特定 agent 接收任何警示，取消勾選所有百分比。Tenant-scope alerts 無法針對單一 agent 停用（它們是整個租戶範圍的）。

### See also

- [Budgets Overview](#budgets-overview).
- [Drop Reasons](#drop-reasons) - 當上限完全達到時會發生的情況。
- [Cost Model](#cost-model) - 正在測量的項目。