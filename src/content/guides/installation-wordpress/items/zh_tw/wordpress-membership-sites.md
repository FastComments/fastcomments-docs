FastComments 透過所謂的 SSO（單一登入）與僅限會員的網站整合。你的會員在你的 WordPress 網站登入後，無需擔心要在 FastComments 建立帳號或使用社群媒體登入就能留言。如果你的會員（包括管理員）已在你的 WordPress 網站登入，他們就能夠留言。你的管理員與版主也可以直接從你的 WordPress 部落格文章中管理留言。

<sup>(Optional)</sup> 別忘了也將你的管理員新增至 [使用者與管理員](https://fastcomments.com/auth/my-account/users)，並將版主新增至 [評論管理員](https://fastcomments.com/auth/my-account/moderate-comments/moderators)，以提升他們的使用體驗並啟用版主的統計追蹤。

SSO 可透過前往外掛面板並點選「SSO Settings」來啟用。

你首先需要啟用網站的「Anyone can Register」功能。

每次使用者透過 [HMAC](https://en.wikipedia.org/wiki/HMAC) 檢視留言串時，所有使用者資訊都會無縫且安全地傳送到 FastComments。

不需要執行任何初次或持續的同步來複製你的會員到 FastComments 伺服器。當他們打開文章並檢視留言串時，這個動作會自動完成。

## 名稱與頭像

當使用者檢視任何留言串時，外掛會自動更新該使用者在 FastComments 上所有留言的顯示名稱與頭像。頭像支援透過 Gravatar 或任何 WordPress 中的頭像管理外掛，因為外掛會呼叫 `get_avatar_url`。

---