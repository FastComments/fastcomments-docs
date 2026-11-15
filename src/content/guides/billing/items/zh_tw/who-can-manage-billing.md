帳單功能僅限兩種使用者使用：

- `Account Owner`，即建立帳戶的使用者。
- 任何具有 `Billing Admin` 權限的使用者。

只有他們可以變更方案、儲存或更換卡片、開啟 `Invoice Settings`、設定 Flex 消費限額，以及儲值。其他任何人嘗試時都會看到 `Only the account owner or a billing admin can change the plan or payment method.`

帳單相關電子郵件，例如試用期中途提醒、付款失敗警告以及停用通知，會寄送給具有 `Billing Admin` 權限的使用者。請將此權限授予應接收這些郵件的人，例如財務人員，他們無需其他儀表板存取權即可收到郵件。試用期結束後若未填寫帳單資訊，所發送的提醒會寄給帳戶上的每一位儀表板使用者。每封郵件都包含取消訂閱連結；選擇退出帳戶通知的使用者將不再收到這些郵件。

發票本身會寄送至 [Invoice Settings](#invoice-settings) 中的 `Billing Email`，該信箱完全不必屬於任何儀表板使用者。

[Billing Analytics](https://fastcomments.com/auth/my-account/analytics/billing) 頁面則需要 `Analytics Admin` 權限，因為它顯示的是使用情況而非付款明細。

若要授予或移除權限，請開啟 `Users` 頁面並編輯該使用者。請參閱 [Updating Administrator Permissions](/guide-users.html#updating-administrator-permissions)。在 Flex 上，請記得每位額外的管理員都是一個計費席位，請參閱 [Admin and Moderator Seats](#admin-and-moderator-seats)。

如果您已登入卻完全看不到 `Billing Info`，或它將您導回首頁，表示您是以留言者帳戶而非儀表板使用者身分登入。請登出並使用擁有該網站的帳戶重新登入。