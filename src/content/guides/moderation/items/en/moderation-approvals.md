With FastComments, an Approved comment is a **visible** comment. To hide a comment, you would un-approve it.

Comments can be automatically approved, or moderators can manually approve each comment. There is also the option to
only require approval of a user's first comment - in which case subsequent comments are automatically approved and do not require moderation.

FastComments has the concept of Verified vs Unverified. Verified comments are either posted in a session verified via email (user is fully logged in, or using SSO)
or were posted in an Unverified state and then later verified manually via email.

The notion of Verified can be completely hidden if desired via customization rules.

Requiring the manual approval of unverified comments can help with spam, as bots will rarely verify their comments via email. In this case
you would want to enable automatic comment approval but enable `Only Auto Approve Verified Comments`.

This can all be configured by administrators from [Moderation Settings](https://fastcomments.com/auth/my-account/moderate-comments/settings).
