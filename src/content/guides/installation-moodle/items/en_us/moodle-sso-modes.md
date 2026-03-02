The plugin supports three SSO modes for integrating Moodle user accounts with FastComments.

#### Secure SSO (Recommended)

User data is signed server-side using HMAC-SHA256 with your API Secret. This is the most secure option and is recommended for production use.

With Secure SSO:

- Usernames, email addresses, and avatars are passed securely to FastComments.
- Moodle site administrators are automatically synced as FastComments moderators.
- Users cannot impersonate other accounts.
- Requires the **API Secret** to be configured in the plugin settings.

#### Simple SSO

User data (name, email address, avatar) is sent client-side without a cryptographic signature. This is easier to set up since it does not require an API Secret, but it is less secure because user data is not verified server-side.

#### None

No SSO integration. Users comment anonymously or must log in to FastComments separately. Use this if you do not want Moodle user accounts linked to comments.