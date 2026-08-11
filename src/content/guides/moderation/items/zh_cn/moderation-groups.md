---
管理员可以被放入组中，以对不同页面或内容类别进行审核。

当管理员属于一个或多个组时，他们在“审核评论”页面只会看到这些组的评论。

例如，假设我们运营一个按类别显示视频的网站。我们可能希望为猫、狗和鹦鹉视频设置不同的审核员，所以[让我们添加这些组](https://fastcomments.com/auth/my-account/moderate-comments/moderation-groups)。

[app-screenshot-start url='/auth/my-account/moderate-comments/moderation-groups?demo=true'; linkUrl='/auth/my-account/moderate-comments/moderation-groups'; selector = '.content'; alt='已为每个视频类别创建的猫、狗和鹦鹉组的审核组列表'; title='审核组页面' app-screenshot-end]

当我们添加审核员时，现在可以选择该审核员所属的一个或多个组：

[app-screenshot-start url='/auth/my-account/moderate-comments/moderator/new?demo=true'; linkUrl='/auth/my-account/moderate-comments/moderator/new'; selector = '.account-block'; alt='添加审核员表单，使用组选择器将审核员分配到一个或多个组'; title='添加审核员并选择组' app-screenshot-end]

最后，评论需要关联到一个或多个组，以便相应的审核员能够看到它们。

这可以通过[添加一些组](https://fastcomments.com/auth/my-account/moderate-comments/moderation-groups)并在评论小部件中指定相应的 `Moderation Group` ID 来设置，
[如此处所述](/guide-customizations-and-configuration.html#moderation-group-ids)。

---