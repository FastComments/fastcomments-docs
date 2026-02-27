Since the login links are essentially passwords, we take the security very seriously.

All login links in our system are set to expire after a certain period of time, and we also have mechanisms in place to detect
the guessing of a login link. Some login links are split into multiple passwords, and if one is guessed,
the other will be invalidated.

### Security Compared to Passwords

With most systems that require a password, you can go through a Forgot Password mechanism
if you have the user's email. This means, if you have access to the user's email account,
it does not matter if the system under attack uses passwords or magic links.

### New IP Login Alerts

When a login occurs from an IP address that has not been seen before for a given account, FastComments sends a security alert email
with the approximate location and IP address. This helps users detect unauthorized access. Note that FastComments does not store
raw IP addresses — only an obfuscated form is stored for security purposes.

### Backup Email for Account Recovery

If you lose access to your primary email, you can use a verified backup email to recover your account. Your backup email works
with all login flows. You can enter it on the forgot username page, use it with magic link login, or type it in the
username/email field for password login.

To set up a backup email, go to [Account Details](https://fastcomments.com/auth/my-account/edit-details) and click
**Define a Backup Email**. Your backup email is only used for account recovery and will not receive notifications.

### Security Compared to MFA

Login Links are less secure than MFA. FastComments now supports two-factor authentication (2FA)
for admin accounts to provide enhanced security. When 2FA is enabled, it is required even when using login links.