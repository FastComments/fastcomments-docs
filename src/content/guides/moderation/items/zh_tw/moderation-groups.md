Moderators can be placed into groups to moderate different pages or categories of content.

When a Moderator belongs to one or more groups, they will only see comments from those groups in the Moderate Comments page.

For example, let's say we run a site that displays videos by category. We may want to have different moderators for Cat, Dog, and Parrot videos, so [let's add those groups](https://fastcomments.com/auth/my-account/moderate-comments/moderation-groups).

[app-screenshot-start url='/auth/my-account/moderate-comments/moderation-groups?demo=true'; linkUrl='/auth/my-account/moderate-comments/moderation-groups'; selector = '.content'; alt='已為每個影片類別建立的貓、狗與鸚鵡群組的審核群組清單'; title='審核群組頁面' app-screenshot-end]

When we add a moderator, we now have the option to select one or more groups the moderator will belong to:

[app-screenshot-start url='/auth/my-account/moderate-comments/moderator/new?demo=true'; linkUrl='/auth/my-account/moderate-comments/moderator/new'; selector = '.account-block'; alt='新增審核者表單，使用群組選擇器將審核者指派至一個或多個群組'; title='新增審核者並選擇群組' app-screenshot-end]

Finally, comments need to be tied to one or more groups so that the correct moderators see them.

This can be setup by [adding some groups](https://fastcomments.com/auth/my-account/moderate-comments/moderation-groups) and then specifying the corresponding `Moderation Group` ids in the comment widget,
[as instructed here](/guide-customizations-and-configuration.html#moderation-group-ids).