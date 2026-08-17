---
預設情況下，每位使用者在同一分鐘內最多可提交 `5 comments`。

此會根據 user id、anon user id 以及 ip address（已雜湊）進行追蹤。

此設定可在小工具自訂頁面上，無需撰寫程式碼即可自訂：

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.max-comments-per-minute'; alt='小工具自訂頁面上每分鐘最大評論欄位，預設為 5'; title='限制每位使用者的評論量' app-screenshot-end]

請注意，如果您使用評論建立 API，可能需要在請求中傳遞使用者原始的 `ip` 位址給我們的後端，以便將速率限制套用於每位使用者，而非全域套用於您的帳號。

---