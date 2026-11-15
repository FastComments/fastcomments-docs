您可能被限流或計費的任何指標都會在 [計費分析頁面](https://fastcomments.com/auth/my-account/analytics/billing) 中報告。

以下是一些被追蹤的指標範例：

- 頁面載入次數(當月)
- API 點數(當月)
- 新儲存的留言數(當月)
- 租戶使用者
- SSO 使用者(無管理員/審核員權限的一般 SSO 使用者)
- SSO 管理員(具有 `isAccountOwner` 或 `isAdminAdmin` 管理員標記的 SSO 使用者 - 與一般 SSO 使用者分開計費)
- SSO 審核員(具有 `isCommentModeratorAdmin` 標記的 SSO 使用者 - 與一般 SSO 使用者分開計費)
- 審核員
- 網域

對於 `Creators` 和 `Pro` 方案,這些指標僅作為參考,讓您了解是否接近或已達到帳戶的限制。

對於 `Flex` 方案,此頁面將顯示**下一期**帳單目前使用量的費用明細。這讓您可以在帳單產生之前輕鬆了解計費方式。您可以在 [帳單歷史記錄](https://fastcomments.com/auth/my-account/billing-history) 中查看過去的帳單。每一行的計算方式以及帳單的扣款時間,均在 [計費指南](/guide-billing.html#how-flex-usage-is-measured) 中說明。

`Page Loads`、`API Credits` 和 `New Comments Stored` 指標可能有一分鐘的延遲。其他指標則為即時。

這些資訊會按月儲存一年,或在您刪除租戶之前作為歷史記錄保留,但 UI 僅會顯示目前的期間。