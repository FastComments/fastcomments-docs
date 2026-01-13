---
預設情況下，每位使用者在同一分鐘內最多可以提交 `5 comments`。

這是透過 user id、anon user id 與 ip address (hashed) 來追蹤。

這可以在小工具自訂頁面上無需撰寫程式地進行自訂：

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.max-comments-per-minute'; title='Limiting Comment Volume Per User' app-screenshot-end]

請注意，如果您正在使用 comment creation API，可能想在發送到我們後端的請求中傳遞使用者原始的 `ip` 位址，以便速率限制能依使用者套用，而非全域套用到您的帳戶。
---