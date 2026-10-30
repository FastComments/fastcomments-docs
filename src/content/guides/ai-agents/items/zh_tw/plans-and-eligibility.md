AI 代理在 **Flex** 與 **Pro** 計畫中可用。Creator 計畫不包含它們。

### Plan-level limits

每個計畫層級設定：

- **預設每日與每月預算上限。** 您可以針對每個代理降低此上限；若要提升每帳戶的上限，需升級至上限更高的計畫。請參閱 [Budgets Overview](#budgets-overview)。

精確的數字顯示於 [pricing page](https://fastcomments.com/traffic-pricing) 與您的帳戶計費頁面上。它們也會直接顯示在代理編輯表單中，讓您不必離開表單即可查看上限。

FastComments Pro 包含每月 $200 美元的 AI 使用額度。Flex 以每百萬個 token $20 美元的費率計費，適用於所有模型（目前為 GLM 5.2 或 gpt-oss-120B）。

### Billing must be valid

AI 代理僅在租戶的 **有效帳單資訊** 存在時才會運行。若付款方式失效，所有代理將被暫停，且 AI 代理頁面會顯示橫幅，指示擁有 **Billing Admin** 角色的使用者更新帳單。帳單恢復後，代理會自行恢復運作——不會重新執行或補填在中斷期間觸發的事件。

這是一項嚴格的前提條件：Token 消費會向您的帳戶計費，因此平台在沒有有效付款方式時不會發送任何 LLM 呼叫。

### Who can manage agents

代理管理頁面受 **Customization Admin** 儀表板角色限制。**Comment Moderator Admins** 可以審核並決定批准（請參閱 [Approval Workflow](#approval-workflow)），但無法建立或編輯代理。**Billing Admins** 會收到 [budget alert emails](#budget-alerts)，即使他們沒有代理存取權限。