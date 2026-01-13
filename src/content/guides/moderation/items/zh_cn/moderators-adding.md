对于管理员，在 Comment Moderation 页面顶部有一个 Add Moderators 按钮。

[app-screenshot-start url='/auth/my-account/moderate-comments?filter=&text-search=&page=1&count=3&demo=true'; linkUrl='/auth/my-account/moderate-comments'; selector = '.moderation-settings-options'; title='Comment Moderation Setting Buttons' app-screenshot-end]

如果您已经有版主，该按钮将显示 "Edit Moderators"。

让我们来看一下 "Add a Moderator" 页面。

[app-screenshot-start url='/auth/my-account/moderate-comments/moderator/new'; selector = '.account-block'; title='The Add a Moderator Page' app-screenshot-end]

要添加版主，所需的只是姓名和电子邮件。

如果该电子邮件与现有的 FastComments 帐户关联，他们将通过电子邮件被邀请以版主身份加入您的帐户。

如果所给的电子邮件未与现有的 FastComments 帐户关联，将为他们创建一个新的帐户。

邀请链接将发送给版主，该链接可以自动让他们登录。将来如果他们想登录，他们只需访问
<a href="https://fastcomments.com/auth/login" target="_blank">登录页面</a> 并输入您之前提供的姓名/电子邮件。这将向他们发送一个登录链接。

除非他们登出，否则他们将保持登录状态三十天。