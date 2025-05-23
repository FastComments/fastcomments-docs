[related-parameter-start name = 'simpleSSO'; type = 'FastCommentsSSOSimple'; typeLink = 'https://github.com/FastComments/fastcomments-typescript/blob/main/src/fast-comments-comment-widget-config.ts#L14' related-parameter-end]

With Simple SSO, we can provide the commenting widget with information about the user so that they don't have to enter their username or email to comment.

We can configure Simple SSO as follows:

[code-example-start config = {simpleSSO: { username: "Bob", email: "bob@example.com", avatar: "https://example.com/bob.png", websiteUrl: "https://example.com/profiles/bob", displayName: "Bob's Name", displayLabel: "VIP User", loginURL: 'https://example.com/login', logoutURL: 'https://example.com/logout', badgeConfig: { badgeIds: ['badge-id-1', 'badge-id-2'], override: false } }}; linesToHighlight = [6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18]; title = 'Simple SSO'; code-example-end]

The user will be logged in, and will create an SSO User behind the scenes. The user will have `createdFromSimpleSSO` set to `true` if fetched from the API.

Notes: 

- Email is the unique identifier for Simple SSO.
- Providing an email with Simple SSO is not required, however by default their comments will show as "Unverified". <b>If no email is provided, the user cannot be fully authenticated.</b>
- **NEW** Since Jan 2022: Usernames do not have to be unique across all of fastcomments.com
- Simple SSO can automatically create and update SSO users, if an email is provided, and the user was not originally created from Secure SSO.
- You can specify badges for the user with the `badgeConfig` property. The `badgeIds` array contains the IDs of badges to associate with the user. If `override` is set to `true`, it will replace all existing badges displayed on comments; if `false` it will add to existing badges.
