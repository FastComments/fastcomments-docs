There are two ways to ban users from commenting on your site with FastComments.

The first is if you already know their email, you can enter it on the <a href="https://fastcomments.com/auth/my-account/moderate-comments/banned-users" target="_blank">被禁止用户</a> page.

[app-screenshot-start url='/auth/my-account/moderate-comments/banned-users'; selector = '.content .account-block'; alt='在“审核评论”下的被禁止用户列表，显示被禁止的电子邮件地址以及添加新禁令的按钮'; title='被禁止用户页面' app-screenshot-end]

This page can be accessed via Moderate Comments -> Banned Users

When we go to ban a user, we can pick a type, either Permanent or Permanent Shadow Ban:

[app-screenshot-start url='/auth/my-account/moderate-comments/banned-users/new'; selector = '.content .account-block'; alt='新禁令表单，包含电子邮件字段以及永久或永久影子禁令的类型选择'; title='禁止用户' app-screenshot-end]

The second way to ban a user is by clicking the ban button that is placed on each comment on the Comment Moderation page.

When we click the ban button, you'll be presented with some options, where we can specify the ban type and duration.

### 电子邮件别名

When banning a user by email, FastComments automatically ignores `+` aliases. For example, banning `user+alias@gmail.com` will
also ban `user@gmail.com` and any other `+` variation of that address, such as `user+other@gmail.com`.

### 影子禁令

A shadow-ban is a type of ban that makes it appear that the user's comment or vote was saved successfully, when in fact it was not. This may be
desirable in certain situations.

### 通过 IP 地址禁令

Unless a tenant wishes to opt out, FastComments supports banning via IP by storing a hashed version of the commenter's IP address.

### 搜索被禁止用户

Once your list grows past a page or two, you can narrow it with the search row above the table.

[app-screenshot-start url='/auth/my-account/moderate-comments/banned-users'; selector = '.content .filter-form'; alt='被禁止用户页面上的搜索行，包含“搜索依据”下拉框、“匹配方式”下拉框和“值”输入框'; title='搜索被禁止用户' app-screenshot-end]

There are three controls:

- **搜索依据** picks which field to look in: Any Field, Email, Name, Banned By, or Banned For Saying. The last four correspond to the columns of the same name in the table.
- **匹配方式** picks how to compare. **Contains** finds your value anywhere in the field, and **Equals** matches the whole field.
- **值** is the text to look for.

Every field is matched without regard to case, so searching for `SPAMMER@EXAMPLE.COM` finds a ban stored as `spammer@example.com`.

以下是一些值得了解的事项：

- **禁言内容** searches the text of the comment that got the user banned. This is how you find everyone banned over a particular phrase.
- **禁令执行者** searches the name of the moderator who issued the ban, which is useful for reviewing another moderator's decisions.
- Wildcard bans are stored with their `*`, so a **Contains** search for `bademail.com` finds a `*@bademail.com` ban.
- **姓名** matches the name shown in the Name column, so it finds a user even if they have changed their name since being banned, and even if you created the ban by entering an email address and no name was recorded at the time. The name recorded on the ban still matches too, so searching for either the old or the current name works.
- **任意字段** searches the email, name, banned-by moderator, and banned comment text together.

Your search is part of the page URL, so you can share a filtered list with other moderators the same way you share other moderation links. Paging through results keeps the search applied, starting a new search returns you to the first page, and **Clear** returns to the full list.