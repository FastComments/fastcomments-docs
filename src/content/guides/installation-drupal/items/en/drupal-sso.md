FastComments integrates with Drupal's user system through SSO, or single-sign-on. Your users sign in to your Drupal site, and the module passes their identity to FastComments automatically. No extra accounts to create, no initial sync to run.

The module supports three SSO modes, set under `Administration > Configuration > Content > FastComments`.

### None

No SSO. Users comment as guests or create a FastComments account. Use this if your site is public and you don't need to tie comments to Drupal users.

### Simple

Passes the Drupal user's name, email, and avatar to FastComments without server-side verification. No API Secret needed. Good for internal or low-risk sites.

### Secure (recommended)

Uses [HMAC-SHA256](https://en.wikipedia.org/wiki/HMAC) to verify each user identity with FastComments. This is the mode you want when you have an API Secret configured, and it's the only mode that prevents a visitor from impersonating another user.

User identity is passed to FastComments each time a user views a comment thread. There is no initial or continuous sync that needs to run.

<sup>(Optional)</sup> Add your administrators to [Users & Administrators](https://fastcomments.com/auth/my-account/users) and moderators to [Comment Moderators](https://fastcomments.com/auth/my-account/moderate-comments/moderators) to improve their experience and enable stat tracking for moderators.

For a deeper look at how SSO works, see the [SSO section](/guide-customizations-and-configuration.html#sso) of the customization docs.
