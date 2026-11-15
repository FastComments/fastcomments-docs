[Invoice Settings](https://fastcomments.com/auth/my-account/invoice-settings) 頁面可透過 Billing Info 上的 `Invoice Settings` 按鈕進入，用於控制發票上顯示的內容以及發票的寄送位置。只有 `Account Owner` 或 `Billing Admin` 可以開啟此頁面。

- `Company Name`:您的帳戶名稱,顯示於儀表板、電子郵件和團隊邀請中。這不是帳單名稱。
- `Billing Email`:發票和帳單通知的寄送地址。如果帳戶擁有者不應收到這些通知,請將此設定為財務或會計信箱。這不會變更任何使用者的登入電子郵件。
- `Invoice Currency`:您的發票所使用的幣別。請參閱 [Currencies](#currencies)。
- `Billing Address`:印在發票上的 `Name`、`Address`、`City`、`State / Province / Region`、`Postal Code` 和 `Country`。當您儲存卡片時,Billing Info 表單上也會出現相同的欄位;兩者寫入的是同一個位置。
- `VAT / Tax ID`:選填。會印在您的發票上。

點擊 `Save Changes`。頁面會顯示 `Invoice settings updated successfully.` 以確認,變更將套用於下一張發票。現有的發票不會被更改。

### VAT 與稅籍編號

在輸入編號之前,請先選擇您的帳單 `Country`,因為國家會決定編號的類型:歐盟成員國為 EU VAT 號碼,英國為 UK VAT 號碼,澳洲為 ABN,加拿大為 GST/HST 號碼,美國為 EIN,其他 Stripe 支援的國家也依此類推。

編號會在您儲存時進行驗證。如果該編號對所選國家無效,頁面會顯示提示且該編號不會被儲存,但您的其他設定仍會被儲存。如果我們無法為您所在的國家記錄稅籍編號,頁面也會告知您。

如果您的帳單地址位於歐盟,Flex 發票將以 0% VAT 的反向徵稅交易方式開立,且您的 VAT 號碼在儲存後會顯示在發票上,方便您的會計師處理您這邊的 VAT。