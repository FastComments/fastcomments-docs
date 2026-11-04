對於您可能不希望成為 `Administrators` 的開發人員，請考慮建立一個具有以下權限的 `Administrator` 使用者：

1. Analytics Admin
2. Customizations Admin
3. Data Management Admin
4. Comment Moderation Admin
5. API/SSO Admin

這組權限將提供開發人員設定 FastComments 所需的一切，以及確保系統正常運作所需的系統可見性。

這些權限的理由如下：

1. **Analytics Admin**：可用於除錯 FastComments 的使用情況。
2. **Customizations Admin**：需要此權限才能為留言小工具套用自訂樣式。
3. **Data Management Admin**：需要此權限才能管理匯入與匯出，以及設定 webhooks。
4. **Comment Moderation Admin**：至少在設定階段需要此權限才能查看留言資料。
5. **API/SSO Admin**：這將允許他們直接從我們的平台取得 API 金鑰。我們認為這比由 `Administrator` 代為複製並透過電子郵件傳送 API Secret 更安全，因為電子郵件可能不太安全。