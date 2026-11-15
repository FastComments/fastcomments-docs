Flex 發票是一份逐項列出各類用量的清單，每一項都顯示數量與費率，例如 `213,000 Page Loads. ($1.00 per 100,000 Page Loads)`。費率公佈在[定價頁面](https://fastcomments.com/traffic-pricing)以及您的[帳單資訊](https://fastcomments.com/auth/my-account/billing-info)頁面上。本節將說明各項數量的含義。

### 按月累計

這些項目每月重置，並從日曆月的第一天計算到最後一天（UTC）：

- **Page Loads（頁面載入次數）**：留言小工具或留言串在您的頁面上載入的次數。留言小工具每次載入都會計入，無論它顯示多少則留言。不會按留言串或按頁面收費。
- **API Credits（API 點數）**：使用您的 API 金鑰呼叫 REST API 的次數。
- **Small Widgets Credits（小型小工具點數）**：小型小工具的載入次數，例如留言計數與徽章。
- **LLM Tokens（LLM 權杖）**：[AI Agents](/guide-ai-agents.html) 與其他 LLM 功能所使用的權杖數量。

### 月底結算

這些是建立發票時所擷取的快照：

- **Comments Stored（已儲存留言數）**：您帳戶中的留言總數，而不僅是本月新增的留言。刪除留言會減少此數量，但已刪除且有回覆的留言會保留為佔位符，以確保回覆仍然有意義。
- **SSO Users（SSO 使用者數）**：您的網站所建立的 SSO 使用者總數，包括很久以前只留言過一次的人。如果這是您帳單中佔比最大的部分，請向支援團隊詢問改為按每月活躍使用者計費的方案，該方案只計算當月有登入的使用者。
- **Moderators（版主）**、**Admins（管理員）**、**SSO Moderators（SSO 版主）**與 **SSO Admins（SSO 管理員）**：席位，詳見 [Admin and Moderator Seats](#admin-and-moderator-seats)。第一位管理員免費。
- **Domains（網域）**：您帳戶中的網站。第一個網站免費，之後每增加一個都按月計費，只要它仍在帳戶中就會持續收費。子網域只有在您將它新增為獨立網站時才會計入；一個頂級網域已涵蓋其子網域。請參閱 [Multiple Sites](/guide-multiple-sites.html#domain-limits)。
- **Managed Tenants（受管理的租戶）**：由此帳戶管理的子帳戶（如果有的話），以及它們各自的用量會以獨立項目列出。

### 最低消費

如果各項加總低於 Flex 最低消費金額，發票將補足至最低金額。只有一位管理員和一個網域的小型網站只需支付最低消費金額。

### 預覽發票

[帳單分析](https://fastcomments.com/auth/my-account/analytics/billing)頁面會顯示下一張發票的每一個項目以及當天的數據，因此不會有任何意外。頁面載入、API 與小型小工具的計數可能會延遲最多一分鐘；其餘數據則為即時更新。

### 其他貨幣

Flex 用量以美元計價。如果您的發票貨幣是其他幣別，每個項目會按發票建立當天的匯率換算，因此相同的用量在不同月份的金額可能略有不同。請參閱 [Currencies](#currencies)。