在 FastComments 中，有兩種方式可以禁止使用者在您的網站發表評論。

第一種是如果您已經知道他們的電子郵件，您可以在 <a href="https://fastcomments.com/auth/my-account/moderate-comments/banned-users" target="_blank">被封鎖的使用者</a> 頁面輸入該電子郵件。

[app-screenshot-start url='/auth/my-account/moderate-comments/banned-users'; selector = '.content .account-block'; title='The Banned Users Page' app-screenshot-end]

此頁面可以透過 Moderate Comments -> Banned Users 存取

當我們封鎖使用者時，可以選擇類型：永久（Permanent）或永久隱形封鎖（Permanent Shadow Ban）：

[app-screenshot-start url='/auth/my-account/moderate-comments/banned-users/new'; selector = '.content .account-block'; title='Banning a User' app-screenshot-end]

第二種封鎖使用者的方式，是在 Comment Moderation 頁面中，點擊置於每則評論上的封鎖按鈕。

當我們點擊封鎖按鈕時，會出現一些選項，可在其中指定封鎖類型與期限。

### Email Aliases

當透過電子郵件封鎖使用者時，FastComments 會自動忽略 `+` 別名。例如，封鎖 `user+alias@gmail.com` 也會封鎖 `user@gmail.com` 以及該地址的任何其他 `+` 變體，例如 `user+other@gmail.com`。

### Shadow Bans

隱形封鎖是一種會讓使用者以為其評論或投票已成功儲存，但實際上並未儲存的封鎖類型。在某些情況下，這是可取的做法。

### Banning Via IP Address

除非租戶選擇退出，FastComments 支援透過 IP 進行封鎖，方法是儲存評論者 IP 地址的雜湊版本。