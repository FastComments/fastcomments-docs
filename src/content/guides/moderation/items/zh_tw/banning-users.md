在 FastComments 中有兩種方式可以禁止使用者在您的網站上發表評論。

第一種是如果您已經知道他們的電子郵件，您可以在 <a href="/auth/my-account/moderate-comments/banned-users" target="_blank">被封鎖使用者</a> 頁面輸入。

[app-screenshot-start url='/auth/my-account/moderate-comments/banned-users'; selector = '.content .account-block'; title='被封鎖使用者頁面' app-screenshot-end]

可從 Moderate Comments -> Banned Users 進入此頁面

當我們要封鎖使用者時，可選擇封鎖類型：永久或永久隱形封鎖（Permanent Shadow Ban）：

[app-screenshot-start url='/auth/my-account/moderate-comments/banned-users/new'; selector = '.content .account-block'; title='封鎖使用者' app-screenshot-end]

第二種方式是，在 Comment Moderation 頁面中點擊每則評論旁的封鎖按鈕來封鎖使用者。

當我們點擊封鎖按鈕時，會顯示一些選項，可指定封鎖類型與期間。

### 隱形封鎖

隱形封鎖（Shadow-ban）是一種封鎖方式，會讓使用者看起來其評論或投票已成功儲存，實際上並未儲存。在某些情況下，這可能是理想的做法。

### 透過 IP 位址封鎖

除非租戶選擇退出，FastComments 支援透過 IP 封鎖，方法是儲存留言者 IP 位址的雜湊版本。