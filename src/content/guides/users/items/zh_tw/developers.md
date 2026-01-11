對於您可能不想設為 `Administrators` 的開發人員，請考慮建立一個具有下列權限的 `Administrator` 使用者
with the following permissions:

1. Analytics Admin
2. Customizations Admin
3. Data Management Admin
4. Comment Moderation Admin
5. API/SSO Admin

這組權限會提供開發人員設定 FastComments 所需的一切，
以及確保系統正常運作所需的可見性。

設定這些權限的理由如下：

1. **Analytics Admin**: 這可以用來偵錯 FastComments 的使用情況。
2. **Customizations Admin**: 這是套用評論元件自訂樣式所需要的。
3. **Data Management Admin**: 這是管理匯入與匯出，以及設定 webhooks 所需要的。
4. **Comment Moderation Admin**: 這在設定期間至少需要，以便查看評論資料。
5. **API/SSO Admin**: 這將允許他們直接從我們的平台擷取 API 金鑰。我們認為
this more secure than an `Administrator` copying it for them and sending the API Secret via email which
   這樣做可能不是很安全。