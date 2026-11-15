付款失敗的原因可能是卡片過期、已更換、餘額不足，或銀行拒絕扣款。FastComments 會給您時間在一切停止運作之前修復問題，並通知能夠處理的人員。

### 會發生什麼事

1. **扣款失敗。** 發票在 [Billing History](https://fastcomments.com/auth/my-account/billing-history) 中保持 `Unpaid` 狀態，並顯示 `Pay Now` 按鈕。在固定方案上，Stripe 可能會重試扣款。在 Flex 上，FastComments 每晚會重試任何超過寬限期的未付款發票。
2. **發送警告。** FastComments 每晚檢查每個帳戶。如果 Flex 發票在寬限期後仍未付款且重試失敗，或固定方案的訂閱因無法收款而結束，則每個具有 `Billing Admin` 權限的使用者都會收到電子郵件 `Your FastComments Tenant Will Be Deactivated Soon!`，其中包含日期，並且該帳戶被排定在一個寬限期後停用。在此期間小工具仍可正常運作。
3. **停用。** 如果在排定日期之前沒有任何改變，帳戶將被停用：您網站上的留言會被隱藏，訪客會看到 `This FastComments account's billing information is more than sixty days out of date. Comments hidden until the issue is resolved.` 帳務管理員會再次收到電子郵件。您的資料會被保留。

大多數帳戶的寬限期為一週，超過一年的帳戶為兩週。在 Flex 上，寬限期會套用兩次，一次在警告之前，一次在警告與停用之間，因此從扣款失敗到留言被隱藏之間至少有兩週的時間。

### 如何修復

在 [Billing Info](https://fastcomments.com/auth/my-account/billing-info) 頁面儲存一張有效的卡片。僅此操作即可立即重新啟用帳戶並取消任何排定的停用；然後未付款的發票會使用新卡片收款，在固定方案上會立即進行，在 Flex 上則在每晚檢查時進行。您也可以直接在 Billing History 中使用 `Pay Now` 立即支付發票。

如果檔案中的卡片沒有問題且失敗只是暫時性的，您無需執行任何操作：每晚檢查會注意到已付款的發票或有效的訂閱，取消停用，並發送電子郵件給帳務管理員以確認。已停用的帳戶也會以相同方式重新啟用。

### 同時有多張發票

如果您的卡片在幾個月內無效然後修復了，每個未付款的月份會分別收款，因此您可能會在同一天看到多筆扣款。請在 Billing History 中檢查每張發票的帳單期間：它們是針對不同月份的，並非重複項目。

### 付款後仍看到帳務警告？

警告屬於特定帳戶。如果您已付款但某個網站仍顯示 `billing information is out of date`，該網站上的小工具很可能使用的是與您付款的帳戶不同的帳戶。當網站被設置兩次，或者當 `fastcomments.com` 上有一個帳戶而 EU 部署環境 `eu.fastcomments.com` 上有另一個帳戶時（兩者分別計費），就會發生這種情況。請將您頁面程式碼中的 `tenantId` 與 `Settings -> API/SSO` 下的進行比對，或[聯絡支援團隊](https://fastcomments.com/auth/my-account/help)，我們會為您解決問題。