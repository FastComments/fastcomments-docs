[related-parameter-start name = 'simpleSSO'; type = 'FastCommentsSSOSimple'; typeLink = 'https://github.com/FastComments/fastcomments-typescript/blob/main/src/fast-comments-comment-widget-config.ts#L14' related-parameter-end]

With Simple SSO, we can provide the commenting widget with information about the user so that they don't have to enter their username or email to comment.

We can configure Simple SSO as follows:

[code-example-start config = {simpleSSO: { username: "Bob", email: "bob@example.com", avatar: "https://example.com/bob.png", websiteUrl: "https://example.com/profiles/bob", displayName: "Bob's Name", displayLabel: "VIP User", loginURL: 'https://example.com/login', logoutURL: 'https://example.com/logout', badgeConfig: { badgeIds: ['badge-id-1', 'badge-id-2'], pageBadgeIds: ['badge-id-3'], override: false } }}; linesToHighlight = [6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19]; title = 'Simple SSO'; code-example-end]

The user will be logged in, and will create an SSO User behind the scenes. The user will have `createdFromSimpleSSO` set to `true` if fetched from the API.

Notes: 

- Email is the unique identifier for Simple SSO.
- Providing an email with Simple SSO is not required, however by default their comments will show as "Unverified". <b>If no email is provided, the user cannot be fully authenticated.</b>
- **NEW** Since Jan 2022: Usernames do not have to be unique across all of fastcomments.com
- Simple SSO can automatically create and update SSO users, if an email is provided, and the user was not originally created from Secure SSO. If a Simple SSO payload names a user created by Secure SSO or the API, that user is left untouched and the visitor gets a separate user.
- You can specify badges for the user with the `badgeConfig` property. The `badgeIds` array contains the IDs of global badges to associate with the user. The `pageBadgeIds` array contains badge IDs scoped to the current page (`urlId`) — these badges are only displayed on the page where they were assigned. If `override` is set to `true`, it will replace existing displayed badges (global and page-scoped are overridden independently); if `false` it will add to existing badges.

#### Simple SSO Settings

Simple SSO is always available. Two settings control how much FastComments trusts it. You can find them by searching for SSO in a <a href="https://fastcomments.com/auth/my-account/customize-widget" target="_blank">customization rule</a>. Both are off by default, and they apply whether or not "Require SSO To View Comments" is on.

Accounts that were already using Simple SSO when these settings were added have both turned on, so nothing changed for them.

Only the customization rule that best matches a page applies, so if you have more than one rule, set these on each of them. A new rule starts with them on if any of your other rules has them on.

##### Auto Verify Comments via Simple SSO

When this is on, comments from Simple SSO users are verified right away.

When this is off, comments from Simple SSO users show as "Unverified" until the user clicks a link we email to the address in the Simple SSO payload. If you only auto approve verified comments, they wait for that link too. If there is no email in the payload, the comment stays unverified.

This setting only affects comments. Votes from Simple SSO users are always counted right away.

##### Allow Simple SSO Admins

When this is on, a Simple SSO user can be an admin or moderator of your account:

- A Simple SSO user that has admin or moderator permissions keeps them.
- A Simple SSO user with the same email as one of your administrators or moderators picks up their permissions.

When this is off, Simple SSO does not grant these permissions. A Simple SSO payload naming a user that has them gets a separate user with no permissions, and the original user is left untouched.

Simple SSO payloads are not signed, so when this is on, anybody who knows an administrator's email or user id can act as them. If that matters for your site, leave this off and use Secure SSO for your administrators.

Users created by Secure SSO or the API can't be reached by Simple SSO, whether or not this is on.
