---
对于管理员，在评论审核页面的顶部有一个“添加版主”按钮。

[app-screenshot-start url='/auth/my-account/moderate-comments?filter=&text-search=&page=1&count=3&demo=true'; linkUrl='/auth/my-account/moderate-comments'; selector = '.moderation-settings-options'; alt='评论审核页面顶部的一排按钮，包括“添加版主”按钮'; title='评论审核设置按钮' app-screenshot-end]

如果您已经有版主，此按钮会显示“编辑版主”。

让我们看看“添加版主”页面。

[app-screenshot-start url='/auth/my-account/moderate-comments/moderator/new'; selector = '.account-block'; alt='添加版主页面，仅要求新版主的姓名和电子邮件，然后发送邀请'; title='添加版主页面' app-screenshot-end]

要添加版主，只需提供姓名和电子邮件。

如果该电子邮件已关联到现有的 FastComments 账户，他们将通过电子邮件被邀请加入您的账户并成为版主。

如果提供的电子邮件未关联到现有的 FastComments 账户，将为其创建一个新账户。

邀请链接将发送给版主，链接可自动登录。将来如果他们想登录，只需访问<a href="https://fastcomments.com/auth/login" target="_blank">登录页面</a>并输入您之前提供的姓名/电子邮件，即可收到登录链接。

除非他们登出，否则将在登录后保持三十天。
---