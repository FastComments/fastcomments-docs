There are two ways to ban users from commenting on your site with FastComments.

The first is if you already know their email, you can enter it on the <a href="/auth/my-account/moderate-comments/banned-users" target="_blank">banned users</a> page.

[app-screenshot-start url='/auth/my-account/moderate-comments/banned-users'; selector = '.content .account-block'; title='The Banned Users Page' app-screenshot-end]

This page can be accessed via Moderate Comments -> Banned Users

When we go to ban a user, we can pick a type, either Permanent or Permanent Shadow Ban:

[app-screenshot-start url='/auth/my-account/moderate-comments/banned-users/new'; selector = '.content .account-block'; title='Banning a User' app-screenshot-end]

The second way to ban a user is by clicking the ban button that is placed on each comment on the Comment Moderation page.

When we click the ban button, we are presented with some options, where we can specify the ban type and duration:

[app-screenshot-start url='/auth/my-account/moderate-comments?filter=&text-search=&page=1&count=1&demo=true'; linkUrl='/auth/my-account/moderate-comments'; clickSelectors = ['.comment .menu', '.comment .menu-content .comment-ban']; selector = '.content .comment'; title='Clicking Ban' app-screenshot-end]

### Shadow Bans

A shadow ban is a type of ban that makes it appear that the user's comment or vote was saved successfully, when in fact it was not. This may be
desirable in certain situations.

### Banning Via IP Address

Unless a tenant wishes to opt out, FastComments supports banning via IP by storing a hashed version of the commenter's IP address.
