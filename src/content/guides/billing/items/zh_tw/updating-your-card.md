沒有獨立的「編輯卡片」頁面。若要更換檔案中的卡片，請開啟 [Billing Info](https://fastcomments.com/auth/my-account/billing-info)，保持方案不變，在 `Payment method` 下方輸入新卡片資訊，然後點擊 `Save Info`。

新卡片將成為所有未來發票的預設付款方式。儲存時不會產生任何扣款，除非您有未付款的發票，在這種情況下，該發票將使用新卡片收款：固定方案會立即扣款，Flex 則會在隔天扣款。當您的帳務狀態正常時，表單會顯示 `You won't be charged now unless you change plans.`。如果您的固定方案訂閱已到期，儲存卡片將啟動新的訂閱並收取第一期的費用，與首次註冊時相同。

如果您的銀行要求您確認卡片，點擊儲存後會開啟確認視窗。只有銀行核准後，卡片才會被儲存。

### 誰可以執行此操作

只有 `Account Owner` 或具有 `Billing Admin` 權限的使用者可以儲存卡片或變更方案。其他使用者會看到 `Only the account owner or a billing admin can change the plan or payment method.`。如果 `Settings` 選單呈灰色或帳務頁面將您重新導向至首頁，表示您是以留言者或沒有帳務存取權限的使用者身分登入，而非帳戶擁有者。請使用建立網站的帳戶登入，或請擁有者授予您權限；請參閱 [Who Can Manage Billing](#who-can-manage-billing)。

### 卡片被拒絕或封鎖

如果卡片被拒絕，頁面會顯示 `Your card was declined. Check that the entered information is correct.`，且不會儲存任何內容。請檢查卡號、有效期限、CVC 和帳單地址，然後再試一次。如果銀行拒絕了確認，您會看到 `Your bank did not approve the payment. Try again, or use a different card.`

有些銀行會封鎖線上定期付款，或在一次拒絕嘗試後封鎖卡片。在這種情況下，請嘗試其他卡片，或在 [Billing History](https://fastcomments.com/auth/my-account/billing-history) 中使用 `Pay Now` 支付未結清的發票。如果您的銀行完全不允许自動扣款，請[聯絡客服](https://fastcomments.com/auth/my-account/help)，我們可以改為發送該發票的付款連結。

僅接受信用卡。表單上的 `Country` 清單包含 Stripe 可以扣款的國家；如果缺少您的國家，請聯絡客服。