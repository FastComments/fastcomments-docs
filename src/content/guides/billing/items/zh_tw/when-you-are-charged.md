您的卡片被扣款的金額與時間，取決於您所使用的方案類型。

### 固定方案

當您開始 `Creators` 或 `Pro` 訂閱時，第一個月或第一年的費用會在您點擊 `Pay Now & Start Subscription` 時立即在瀏覽器中扣款。如果您的銀行要求您確認付款，請在開啟的視窗中完成驗證。只有在付款完成後，訂閱才會開始。

之後，訂閱會在每月的同一天續訂，若您選擇 `Pay Annually` 則是每年的同一天，從您訂閱的那天起算。續訂會自動向檔案中的卡片扣款，發票會寄送到您的帳務電子郵件。您無需採取任何行動。

按年計費比按月計費便宜 15%。Flex 無法按年計費。

### Flex

Flex 採用事後計費：您在日曆月份內使用服務，該月份的發票會在當月最後一天（UTC）建立並立即向您的卡片扣款。帳單日期永遠是當月最後一天，無法更改。

發票僅涵蓋該日曆月份。您註冊當月的用量不會被計費，因此您的第一張發票最早會在註冊後次月的月底開立。

如果您的用量低於 Flex 最低金額，發票上會出現一行 `FastComments Flex. (minimum ... /mo)`，將金額補足至最低金額。目前的最低金額顯示在您的 [Billing Info](https://fastcomments.com/auth/my-account/billing-info) 頁面和[定價頁面](https://fastcomments.com/traffic-pricing)上。

您可以隨時查看下一張發票的金額。`Next Invoice Amount` 顯示在 [Billing Info](https://fastcomments.com/auth/my-account/billing-info) 頁面上，完整的逐項明細則在 [Billing Analytics](https://fastcomments.com/auth/my-account/analytics/billing) 頁面上。計費內容詳見 [How Flex Usage Is Measured](#how-flex-usage-is-measured)。

### 儲存卡片不會扣款

一旦您的帳務設定完成，在不更換方案的情況下儲存或更新卡片絕不會扣款，除非您有未付款的發票。表單會顯示 `You won't be charged now unless you change plans.`，卡片只是儲存在 Stripe 以供下一張發票使用。只有當您尚未設定有效帳務時，表單才會顯示 `Your card will be charged now.`，即使如此，Flex 帳戶也要到月底才會扣款。您帳戶中的任何預付[額度](#credits)會在卡片扣款前先用於抵扣每張發票。

### 受管理的子帳戶

如果您的帳戶管理子帳戶（例如每個網站或環境一個，或白牌租戶），則只有父帳戶會被計費。每個子帳戶的用量會以子帳戶名稱為前綴，作為獨立項目顯示在父帳戶的 Flex 發票上，且子帳戶共用父帳戶的付款方式與帳務狀態。