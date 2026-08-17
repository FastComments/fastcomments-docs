Moderators can be placed into groups to moderate different pages or categories of content.

當審核員可以被分配到群組，以審核不同的頁面或內容類別。

When a Moderator belongs to one or more groups, they will only see comments from those groups in the Moderate Comments page.

當審核員屬於一個或多個群組時，他們只會在「審核評論」頁面看到該群組的評論。

For example, let's say we run a site that displays videos by category. We may want to have different moderators for Cat, Dog, and Parrot videos, so [let's add those groups](https://fastcomments.com/auth/my-account/moderate-comments/moderation-groups).

[app-screenshot-start url='/auth/my-account/moderate-comments/moderation-groups?demo=true'; linkUrl='/auth/my-account/moderate-comments/moderation-groups'; selector = '.content'; alt='已為每個影片類別建立的貓、狗與鸚鵡群組清單'; title='審核群組頁面' app-screenshot-end]

When we add a moderator, we now have the option to select one or more groups the moderator will belong to:

當我們新增審核員時，現在可以選擇審核員所屬的一個或多個群組：

[app-screenshot-start url='/auth/my-account/moderate-comments/moderator/new?demo=true'; linkUrl='/auth/my-account/moderate-comments/moderator/new'; selector = '.account-block'; alt='新增審核員表單，使用群組選擇器將審核員指派至一個或多個群組'; title='新增審核員並選擇群組' app-screenshot-end]

Finally, comments need to be tied to one or more groups so that the correct moderators see them.

最後，評論需要關聯到一個或多個群組，以便正確的審核員能看到它們。

This can be setup by [adding some groups](https://fastcomments.com/auth/my-account/moderate-comments/moderation-groups) and then specifying the corresponding `Moderation Group` ids in the comment widget,
[as instructed here](/guide-customizations-and-configuration.html#moderation-group-ids).