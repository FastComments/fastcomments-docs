版主可以被分配到不同的组，以管理不同页面或内容类别的评论。

当版主属于一个或多个组时，他们在 Moderate Comments 页面只会看到来自这些组的评论。

例如，假设我们运行一个按类别展示视频的网站。我们可能希望为 Cat、Dog 和 Parrot 视频设置不同的版主，所以[我们来添加这些组](https://fastcomments.com/auth/my-account/moderate-comments/moderation-groups)。

[app-screenshot-start url='/auth/my-account/moderate-comments/moderation-groups?demo=true'; linkUrl='/auth/my-account/moderate-comments/moderation-groups'; selector = '.content'; title='审核组页面' app-screenshot-end]

当我们添加一个版主时，我们现在可以选择该版主将属于的一个或多个组：

[app-screenshot-start url='/auth/my-account/moderate-comments/moderator/new?demo=true'; linkUrl='/auth/my-account/moderate-comments/moderator/new'; selector = '.account-block'; title='添加版主并选择组' app-screenshot-end]

最后，评论需要关联到一个或多个组，以便相应的版主能看到它们。

这可以通过[添加一些组](https://fastcomments.com/auth/my-account/moderate-comments/moderation-groups)来设置，然后在评论小部件中指定相应的 `Moderation Group` ids，
[按此处的说明](/guide-customizations-and-configuration.html#moderation-group-ids)。