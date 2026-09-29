When users comment or vote, and they are not logged in, they will be asked to provide their email and username.

In the case of anonymous commenting, sometimes it is desirable to define a default username to reduce the friction
when commenting. This can be done from the Customization UI. Anonymous Commenting must be enabled first.

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.allow-anonymous-comments']; selector = '.default-username-label'; alt='匿名コメントが有効になるとカスタマイズ UI に表示されるデフォルトユーザー名フィールド'; title='デフォルトユーザー名の設定' app-screenshot-end]

#### デフォルトユーザー名の共有

The default username is a shared display name, not an identity. A visitor who keeps the default and enters their email
gets their own account, with the default shown as their public name. Any number of visitors can keep the same default,
so "Anonymous" is never reported as already taken.

Usernames that a visitor types themselves still have to be unique.

If you would rather each visitor get a distinct name without having to think of one, see
[Generate Usernames Automatically](/guide-customizations-and-configuration.html#auto-generate-username).