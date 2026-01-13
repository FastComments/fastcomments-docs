With FastComments SSO Access Control, sometimes referred to as RBAC, users can be restricted to only access certain pages, or comment threads. Additionally,
users can only `@mention` each other in the same group.

## In Detail

We can place `Users` and optionally `Pages` into groups.

When `Users` are placed into groups, and `Limit Comments by SSO User Groups` is enabled in Widget Settings, then users
will only see comments from users in the same groups as them and will only be able to `@mention` users in the same groups.

Additionally, `Pages` can be placed into groups, and then users can only access comments for pages they have access to.

We call this "User-Level" groups versus "Page-Level" groups.

So which one is right for you?

#### Use User-Level Groups if...

- You want to use the same `urlId` value (page URL, or article ID), but restrict comments by group.
- For example, you want to have "New User" and "Veteran User" groups, and they should never see each other's comments on the same pages.

#### Use Page-Level Groups if...

- Your groups have specific pages.
- For example, users in "Public Pages" group should never view articles in the "Top Secret" group.