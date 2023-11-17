By default, FastComments comes with trainable spam detection.

As you moderate comments, and mark them as **Spam**, or mark comments automatically found as **Spam** as **Not Spam**, the spam
detection system will learn from these actions to more accurately determine what you want to be spam.

Comments marked as **Spam** will not be automatically approved, so they will not show until explicitly marked as **Not Spam**.

Spam Detection can be disabled via the Comment Moderation Settings page.

### Different Spam Detectors

FastComments supports three ways of detecting spam:

1. A traditional Naïve-Bayes classifier that is continuously trained, which is shared across all FastComments.com tenants.
2. A traditional Naïve-Bayes classifier that is continuously trained, which is **isolated** to your tenant.
3. Using ChatGPT 4.

Everyone has access to the shared and isolated Naïve-Bayes classifiers.

The ChatGPT 4 option is selectable in the Comment Moderation Settings page if you are on Flex billing, since it bills based
on tokens used.

### Trust Factor

FastComments adjusts the spam filter for a user based on how much they are trusted for the given site.

For example, if administrators have pinned lots of their comments, then probably they are a very trustworthy user. Or, if
they have been a member of the site for a long time and have a lot of comments, then their trust factor may be high as well.

### SSO

Comments posted by SSO users can be considered spam, and will be checked as such. The exception is if the SSO user
has the same email as a tenant user who has one or more of the following permissions:

- Account Owner
- Super Admin
- Comment Moderator Admin

SSO users with these permissions will not have their comments checked for spam.
