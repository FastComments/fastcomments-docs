有兩種方式可以使用 FastComments 禁止使用者對您的網站發表評論。

第一種是如果您已經知道他們的電子郵件，您可以在 <a href="https://fastcomments.com/auth/my-account/moderate-comments/banned-users" target="_blank">被禁用使用者</a> 頁面上輸入它。

[app-screenshot-start url='/auth/my-account/moderate-comments/banned-users'; selector = '.content .account-block'; alt='Moderate Comments 下的被禁用使用者清單，包含被禁用的電子郵件地址以及新增禁用的按鈕'; title='被禁用使用者頁面' app-screenshot-end]

此頁面可透過 Moderate Comments -> Banned Users 存取

當我們要禁用使用者時，我們可以選擇類型，即永久或永久影子禁用：

[app-screenshot-start url='/auth/my-account/moderate-comments/banned-users/new'; selector = '.content .account-block'; alt='新增禁用表單，包含電子郵件欄位以及永久或永久影子禁用的禁用類型選擇'; title='禁用使用者' app-screenshot-end]

第二種禁用使用者的方法是點擊評論審核頁面上每條評論旁的禁用按鈕。

當我們點擊禁用按鈕時，您將看到一些選項，我們可以在其中指定禁用類型和持續時間。

### 電子郵件別名

透過電子郵件禁用使用者時，FastComments 會自動忽略 `+` 別名。例如，禁用 `user+alias@gmail.com` 也會禁用 `user@gmail.com` 以及該地址的任何其他 `+` 變體，例如 `user+other@gmail.com`。

### 影子禁用

影子禁用是一種禁用類型，它會讓使用者看起來其評論或投票已成功保存，但實際上並未保存。在某些情況下這可能是理想的選擇。

### 透過 IP 位址禁用

除非租戶希望退出，FastComments 支援透過儲存評論者 IP 位址的雜湊版本來進行 IP 禁用。

### 搜尋被禁用使用者

當您的清單增長超過一兩頁時，您可以使用表格上方的搜尋列來縮小範圍。

[app-screenshot-start url='/auth/my-account/moderate-comments/banned-users'; selector = '.content .filter-form'; alt='被禁用使用者頁面上的搜尋列，包含 Search By 下拉選單、Match 下拉選單和 Value 輸入框'; title='搜尋被禁用使用者' app-screenshot-end]

有三個控制項：

- **Search By** 選擇要在哪個欄位中搜尋：Any Field、Email、Name、Banned By 或 Banned For Saying。後四個對應於表格中同名的欄位。
- **Match** 選擇比較方式。**Contains** 會在欄位中的任何位置尋找您的值，而 **Equals** 則匹配整個欄位。
- **Value** 是要搜尋的文字。

每個欄位的匹配都不區分大小寫，因此搜尋 `SPAMMER@EXAMPLE.COM` 會找到儲存為 `spammer@example.com` 的禁用。

一些值得了解的事項：

- **Banned For Saying** 會搜尋導致使用者被禁用的評論文字。這是您找出所有因特定短語而被禁用者的方法。
- **Banned By** 會搜尋發出禁用的審核員姓名，這對於審查其他審核員的決定很有用。
- 萬用字元禁用會與其 `*` 一起儲存，因此對 `bademail.com` 進行 **Contains** 搜尋會找到 `*@bademail.com` 的禁用。
- **Name** 匹配 Name 欄位中顯示的姓名，因此即使使用者在被禁用後更改了姓名，即使您在建立禁用時只輸入了電子郵件地址而當時沒有記錄姓名，也能找到該使用者。禁用中記錄的姓名仍然會匹配，因此搜尋舊姓名或目前姓名都可以。
- **Any Field** 會同時搜尋電子郵件、姓名、禁用審核員以及被禁用的評論文字。

您的搜尋是頁面 URL 的一部分，因此您可以像分享其他審核連結一樣，與其他審核員分享篩選後的清單。翻頁瀏覽結果時搜尋條件會保持套用，開始新的搜尋會將您帶回第一頁，而 **Clear** 則會返回完整清單。