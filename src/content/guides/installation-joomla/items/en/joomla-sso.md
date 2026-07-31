FastComments integrates with Joomla's user system through SSO, or single sign-on. Your users sign in to your Joomla site, and the plugin passes their identity to FastComments automatically. No extra accounts to create, and no initial sync to run.

The plugin supports three SSO modes, set with the **SSO Mode** setting.

### None

No SSO. Users comment as guests or with their own FastComments account. Use this if your site is public and you do not need to tie comments to Joomla users.

### Simple

Passes the Joomla user's name, email, and avatar to FastComments without server-side verification. No API Secret needed. Good for internal or low-risk sites.

### Secure (recommended)

Uses [HMAC-SHA256](https://en.wikipedia.org/wiki/HMAC) to verify each user's identity with FastComments. This is the mode you want when you have an API Secret configured, and it is the only mode that prevents a visitor from impersonating another user.

Joomla Super Users are passed to FastComments as administrators, so your site admins get moderation abilities in the comment widget automatically. User identity is passed each time a user views a comment thread, so there is no sync to run.

<sup>(Optional)</sup> Add your administrators to [Users & Administrators](https://fastcomments.com/auth/my-account/users) and moderators to [Comment Moderators](https://fastcomments.com/auth/my-account/moderate-comments/moderators) to improve their experience and enable stat tracking.

For a deeper look at how SSO works, see the [SSO section](/guide-customizations-and-configuration.html#sso) of the customization docs.
