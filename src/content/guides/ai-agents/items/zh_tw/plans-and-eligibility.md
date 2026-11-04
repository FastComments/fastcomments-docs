AI Agents 在 **Flex** 和 **Pro** 方案中可用。Creator 方案不包含此功能。

### 方案層級限制

每個方案層級設定：

- **預設的每日與每月預算上限。** 您可以為每個 agent 調低此上限；若要調高帳戶層級的上限，則需要更高額度的方案。請參閱 [Budgets Overview](#budgets-overview)。

確切數字顯示在[定價頁面](https://fastcomments.com/traffic-pricing)以及您帳戶的帳單頁面上。這些數字也會直接顯示在 agent 編輯表單中，因此您不必離開表單即可查看您的上限。

FastComments Pro 包含每月價值 $200 的 AI 用量。Flex 則以每百萬 tokens $20 的費率計費，適用於所有模型（目前為 GLM 5.2 或 gpt-oss-120B）。

### 帳單必須有效

只有當租戶存有**有效帳單資訊**時，AI Agents 才會執行。如果付款方式失效，所有 agent 都會暫停，且 AI Agents 頁面會顯示橫幅，引導具有 **Billing Admin** 角色的使用者更新帳單資訊。帳單恢復後，agent 會自動恢復執行——不會重播或補發在中斷期間觸發的觸發器。

這是一項硬性前提條件：token 支出會計入您的帳戶，因此若沒有有效的付款方式，平台將不會發送任何 LLM 呼叫。

### 誰可以管理 agent

Agent 管理頁面受 **Customization Admin** 儀表板角色限制。**Comment Moderator Admins** 可以審核並決定核准（請參閱 [Approval Workflow](#approval-workflow)），但無法建立或編輯 agent。**Billing Admins** 無論是否具有 agent 存取權限，都會收到[預算警示電子郵件](#budget-alerts)。