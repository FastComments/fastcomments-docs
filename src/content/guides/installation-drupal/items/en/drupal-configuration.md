All settings live under `Administration > Configuration > Content > FastComments` (`/admin/config/content/fastcomments`).

## Required

- **Tenant ID** - Your FastComments Tenant ID. Find this under [Settings > API/SSO](https://fastcomments.com/auth/my-account/api) ([EU](https://eu.fastcomments.com/auth/my-account/api)).
- **API Secret** - Required for Secure SSO, webhook verification, and page sync. Found under [Settings > API/SSO](https://fastcomments.com/auth/my-account/api) ([EU](https://eu.fastcomments.com/auth/my-account/api)).

## Storing the API Secret

The module reads the API Secret from the first of these sources that has a value:

1. **Key module** - Install [Key](https://www.drupal.org/project/key), create an *Authentication* key, and pick it under **API Secret key** on the settings form. The key can be backed by an environment variable or a file outside the web root.
2. **settings.php** - Add `$config['fastcomments.settings']['api_secret'] = 'your-secret';` to `settings.php` or `settings.local.php`. The secret never enters exported configuration.
3. **The settings form** - Enter it in the **API Secret** field. This stores the value in `fastcomments.settings`, so it ends up in configuration exports. The Drupal status report warns when this is the active source.

The form shows which source is active. Leaving the field blank keeps the stored secret. Tick **Remove the API Secret stored in configuration** to delete it after moving the secret to the Key module or `settings.php`.

## Commenting Style

Pick the widget that matches how you want people to talk on your site.

- **Live Comments** - Real-time threaded comments.
- **Streaming Chat** - Live chat interface, good for events and livestreams.
- **Collab Chat** - Text-selection annotation on the main content area. Visitors highlight text and start a discussion in context.
- **Collab Chat + Comments** - Both collab chat and standard comments on the same page.

## SSO Mode

- **None** - No SSO. Users comment as guests or create a FastComments account.
- **Simple** - Passes Drupal user info (name, email, avatar) to FastComments without server-side verification.
- **Secure** - Uses HMAC-SHA256 to verify Drupal users with FastComments. Recommended when you have an API Secret configured.

See the `Single Sign-On (SSO)` section for details.

## Other Settings

- **CDN URL** - Defaults to `https://cdn.fastcomments.com`.
- **Site URL** - Defaults to `https://fastcomments.com`.
- **Email notifications** - Send an email to a content author when a new comment is posted on their content.

For EU data residency, see the `EU Data Residency` section.
