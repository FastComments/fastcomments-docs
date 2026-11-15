儲值金額將向您帳戶中保存的付款方式收費。如果您尚未保存卡片,點數頁面會顯示 `Add a payment method first` 以及一個 `Go to billing` 按鈕。請先在 [Billing Info](https://fastcomments.com/auth/my-account/billing-info) 頁面保存卡片,然後再回來。

若要新增點數:

1. 開啟 [My Account -> Billing Info](https://fastcomments.com/auth/my-account/billing-info) 並點擊 `Manage Credits`。
2. 在 `Add credit` 下方,選擇其中一個預設金額,或選擇 `Other amount` 並輸入您自己的金額。
3. 點擊 `Add` 按鈕。它會顯示您選擇的金額,例如 `Add $25.00`。
4. 如果您的銀行要求您確認付款,請在開啟的視窗中完成驗證。

付款完成後,頁面會顯示 `Credit added` 以及您的新餘額,且該筆儲值會以 `Credited` 狀態顯示在 `Past top-ups` 中。點擊 `Add more` 可再次儲值。

### 金額與限制

預設金額為您帳單貨幣的 `10`、`25`、`50` 和 `100`。自訂金額每次儲值可為 `5` 到 `10,000` 之間的任意金額,您的貨幣的確切限制會顯示在表單下方。對於沒有小數位的貨幣(例如日圓),預設金額和限制會放大一百倍,以代表大致相同的金額。

儲值一律以您帳戶的帳單貨幣進行,與您的發票使用相同貨幣。頁面會告知您將以哪種貨幣收費。

### 銀行確認(3D Secure)

某些銀行要求您確認線上付款,通常是透過您的銀行應用程式或一次性驗證碼。如果您的銀行有此要求,在您點擊 `Add` 後會開啟一個確認視窗。只有在銀行核准後,您的卡片才會被扣款,您的餘額才會更新。

如果您在完成確認前關閉頁面,該筆儲值會被記錄為 `Pending`。下次您開啟點數頁面時,我們會向 Stripe 查詢付款狀態,並據此將其標記為 `Credited` 或 `Failed`。如果銀行未核准付款,頁面會顯示 `Your bank did not approve the payment. Try again, or use a different card.`,且不會有任何扣款。

### 卡片被拒

如果您的卡片被拒,頁面會顯示 `Your card was declined. Check your card on the billing page, or try another card.` 不會有任何扣款,您的餘額不會改變,且該次嘗試會以 `Failed` 狀態列在 `Past top-ups` 中。請在 [Billing Info](https://fastcomments.com/auth/my-account/billing-info) 頁面更新卡片後再試一次。

為了保護您的卡片,每個帳戶每小時的儲值嘗試次數有限。如果您看到 `Too many top-up attempts. Wait a while and try again.`,請稍候再重試。

付款由 Stripe 處理。FastComments 絕不會儲存您的卡片資訊。