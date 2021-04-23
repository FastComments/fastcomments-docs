Since the login links are essentially passwords, we take the security very seriously.

All login links in our system are set to expire after a certain period of time, and we also have mechanisms in place to detect
the guessing of a login link. Some login links are split into multiple passwords, and if one is guessed,
the other will be invalidated.

### Security Compared to Passwords

With most systems that require a password, you can go through a Forgot Password mechanism
if you have the user's email. This means, if you have access to the user's email account,
it does not matter if the system under attack uses passwords or magic links.

### Security Compared to MFA

Login Links are less secure than MFA, however this is something we aim to offer in the future for
admin accounts.
