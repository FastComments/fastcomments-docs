有兩種方式可以使用 FastComments 禁止使用者在您的網站發表評論。

第一種是如果您已經知道他們的電子郵件，您可以在 <a href="https://fastcomments.com/auth/my-account/moderate-comments/banned-users" target="_blank">封鎖使用者</a> 頁面輸入該電子郵件。

[app-screenshot-start url='/auth/my-account/moderate-comments/banned-users'; selector = '.content .account-block'; title='The Banned Users Page' app-screenshot-end]

此頁面可透過 Moderate Comments -> Banned Users 存取

當我們要封鎖使用者時，我們可以選擇一種類型，永久封鎖 (Permanent) 或 永久隱形封鎖 (Permanent Shadow Ban)：

[app-screenshot-start url='/auth/my-account/moderate-comments/banned-users/new'; selector = '.content .account-block'; title='Banning a User' app-screenshot-end]

第二種封鎖使用者的方式，是在「評論審核」頁面中，每則評論旁都有一個封鎖按鈕，點擊該按鈕即可封鎖。

當我們點擊封鎖按鈕時，會出現一些選項，讓我們可以指定封鎖類型與持續時間。

### 隱形封鎖

隱形封鎖是一種封鎖方式，會讓使用者以為其留言或投票已成功儲存，但實際上並沒有。在某些情況下這樣的做法是可取的。

### 透過 IP 位址封鎖

除非租戶選擇退出，FastComments 支援透過 IP 封鎖，方式是儲存留言者 IP 位址的雜湊後版本。