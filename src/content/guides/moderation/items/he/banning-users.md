There are two ways to ban users from commenting on your site with FastComments.

The first is if you already know their email, you can enter it on the <a href="https://fastcomments.com/auth/my-account/moderate-comments/banned-users" target="_blank">משתמשים חסומים</a> page.

[app-screenshot-start url='/auth/my-account/moderate-comments/banned-users'; selector = '.content .account-block'; alt='רשימת משתמשים חסומים תחת מודרציה של תגובות, עם כתובות האימייל החסומות וכפתור להוספת חסימה חדשה'; title='דף המשתמשים החסומים' app-screenshot-end]

This page can be accessed via Moderate Comments -> Banned Users

When we go to ban a user, we can pick a type, either Permanent or Permanent Shadow Ban:

[app-screenshot-start url='/auth/my-account/moderate-comments/banned-users/new'; selector = '.content .account-block'; alt='טופס חסימה חדש עם שדה אימייל ובחירת סוג החסימה קבוע או חסימה צללית קבועה'; title='חסימת משתמש' app-screenshot-end]

The second way to ban a user is by clicking the ban button that is placed on each comment on the Comment Moderation page.

When we click the ban button, you'll be presented with some options, where we can specify the ban type and duration.

### כינויים של אימייל

When banning a user by email, FastComments automatically ignores `+` aliases. For example, banning `user+alias@gmail.com` will also ban `user@gmail.com` and any other `+` variation of that address, such as `user+other@gmail.com`.

### חסימות צללית

A shadow-ban is a type of ban that makes it appear that the user's comment or vote was saved successfully, when in fact it was not. This may be desirable in certain situations.

### חסימה באמצעות כתובת IP

Unless a tenant wishes to opt out, FastComments supports banning via IP by storing a hashed version of the commenter's IP address.

---