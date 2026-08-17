---
對於管理員而言，在「評論審核」頁面的頂部有一個「新增審核者」按鈕。

[app-screenshot-start url='/auth/my-account/moderate-comments?filter=&text-search=&page=1&count=3&demo=true'; linkUrl='/auth/my-account/moderate-comments'; selector = '.moderation-settings-options'; alt='評論審核頁面頂部的一排按鈕，包含「新增審核者」按鈕'; title='評論審核設定按鈕' app-screenshot-end]

如果您已經有審核者，這個按鈕會顯示「編輯審核者」。

讓我們來看看「新增審核者」頁面。

[app-screenshot-start url='/auth/my-account/moderate-comments/moderator/new'; selector = '.account-block'; alt='「新增審核者」頁面僅要求輸入新審核者的姓名與電子郵件，然後發送邀請'; title='新增審核者頁面' app-screenshot-end]

要新增審核者，只需要提供姓名與電子郵件。

如果該電子郵件已與現有的 FastComments 帳號關聯，系統會透過電子郵件邀請他們加入您的帳號，成為審核者。

如果提供的電子郵件未與任何 FastComments 帳號關聯，系統會為他們建立新帳號。

系統會將邀請連結發送給審核者，該連結可自動登入。未來若他們想要登入，只需前往<a href="https://fastcomments.com/auth/login" target="_blank">登入頁面</a>，並輸入您先前提供的姓名/電子郵件，即可收到登入連結。

只要他們未登出，將會保持登入狀態長達三十天。

---