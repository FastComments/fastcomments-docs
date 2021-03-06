By default, FastComments comes with trainable spam detection.

As you moderate comments, and mark them as **Spam**, or mark comments automatically found as **Spam** as **Not Spam**, the spam
detection system will learn from these actions to more accurately determine what you want to be spam.

Comments marked as **Spam** will not be automatically approved, so they will not show until explicitly marked as **Not Spam**.

Spam Detection can be disabled via the Comment Moderation Settings page.

### SSO

Comments posted by SSO users can be considered spam, and will be checked as such. The exception is if the SSO user
has the same email as a tenant user who has one or more of the following permissions:

- Account Owner
- Super Admin
- Comment Moderator Admin

SSO users with these permissions will not have their comments checked for spam.
