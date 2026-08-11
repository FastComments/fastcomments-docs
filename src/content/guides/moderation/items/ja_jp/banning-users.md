There are two ways to ban users from commenting on your site with FastComments.

The first is if you already know their email, you can enter it on the <a href="https://fastcomments.com/auth/my-account/moderate-comments/banned-users" target="_blank">禁止されたユーザー</a> page.

[app-screenshot-start url='/auth/my-account/moderate-comments/banned-users'; selector = '.content .account-block'; alt='Moderate Comments の下にある禁止されたユーザーリストで、禁止されたメールアドレスと新しい禁止を追加するボタンがあります'; title='禁止されたユーザーのページ' app-screenshot-end]

This page can be accessed via Moderate Comments -> Banned Users

When we go to ban a user, we can pick a type, either Permanent or Permanent Shadow Ban：

[app-screenshot-start url='/auth/my-account/moderate-comments/banned-users/new'; selector = '.content .account-block'; alt='メールフィールドと Permanent または Permanent Shadow Ban の禁止タイプ選択がある新しい禁止フォーム'; title='ユーザーの禁止' app-screenshot-end]

The second way to ban a user is by clicking the ban button that is placed on each comment on the Comment Moderation page.

When we click the ban button, you'll be presented with some options, where we can specify the ban type and duration.

### メールエイリアス

When banning a user by email, FastComments automatically ignores `+` aliases. For example, banning `user+alias@gmail.com` will also ban `user@gmail.com` and any other `+` variation of that address, such as `user+other@gmail.com`.

### シャドウバン

A shadow-ban is a type of ban that makes it appear that the user's comment or vote was saved successfully, when in fact it was not. This may be desirable in certain situations.

### IP アドレスによる禁止

Unless a tenant wishes to opt out, FastComments supports banning via IP by storing a hashed version of the commenter's IP address.

---