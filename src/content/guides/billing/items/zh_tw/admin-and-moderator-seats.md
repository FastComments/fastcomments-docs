在 Flex 方案中，能夠管理或審核您帳戶的人員會以每月席位計費。這是 Flex 帳單中最常讓人意外的一部分，通常是因為席位在沒有人察覺的情況下被建立了。

### 什麼算作一個席位

- **管理員**：在您帳戶的 `Users` 頁面上具有任何管理權限（帳務、自訂、分析、審核等）的使用者。第一位管理員（通常是帳戶擁有者）是免費的；每位額外的管理員都是一個席位。
- **審核員**：在 `Moderators` 頁面上已接受邀請的人員。待處理的邀請不會計費。僅有留言審核權限的使用者會以審核員計費，而非管理員。
- **SSO 管理員**與 **SSO 審核員**：您的網站以 SSO payload 中設有 `isAccountOwner`、`isAdminAdmin` 或 `isCommentModeratorAdmin` 登入的 SSO 使用者。它們的計費費率與一般管理員和審核員相同。請參閱 [SSO User Structure](/guide-api.html#sso-user-structure)。

席位會依電子郵件地址去重，因此同時是管理員和審核員的人，或擁有相同電子郵件的 SSO 使用者與儀表板登入帳號的人，只會以較高的費率計費一次。擁有兩個不同電子郵件地址的人則算兩個席位。

在按每月活躍使用者計費的帳戶上，當月未登入的 SSO 審核員在該月份不會被計費。

### 您未新增的席位

有兩個功能會自動建立席位：

- **SSO 管理員同步。** 當 SSO 使用者以符合您儀表板使用者的電子郵件登入時，該使用者的管理員或審核員權限會被複製到 SSO 使用者。使用 WordPress SSO 時，這代表每位 WordPress 管理員都可以成為 FastComments 的管理員或審核員。您可以在 `Customize` 下的自訂規則中，使用 `Disable Automatic Admin Migration` 選項將其關閉。
- **同一人使用不同的電子郵件。** 如果您的儀表板登入使用一個電子郵件，而您的 SSO 使用者使用另一個，則算兩個席位。在兩者使用相同的電子郵件即可將其合併。

### 檢查與移除席位

[Billing Analytics](https://fastcomments.com/auth/my-account/analytics/billing) 頁面會列出您的 `Moderators`、`SSO Admins` 和 `SSO Moderators` 數量，下一張發票預覽會顯示其費用。請從 `Users` 和 `Moderators` 頁面移除不需要的人員，或從您的 SSO payload 中移除管理員標記。席位是在月底建立發票時計算的，因此在該時間點之前移除的席位不會在該月份被計費。

有關如何管理兩者的方法，請參閱 [Adding Moderators](/guide-users.html#adding-moderators) 與 [Updating Administrator Permissions](/guide-users.html#updating-administrator-permissions)。