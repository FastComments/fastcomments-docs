All settings live under `Administration > Configuration > Content > FastComments` (`/admin/config/content/fastcomments`).

## Required

- **Tenant ID** - Your FastComments Tenant ID. Find this under [Settings > API/SSO](https://fastcomments.com/auth/my-account/api) ([EU](https://eu.fastcomments.com/auth/my-account/api)).
- **API Secret** - Required for Secure SSO, webhook verification, and page sync. Found under [Settings > API/SSO](https://fastcomments.com/auth/my-account/api) ([EU](https://eu.fastcomments.com/auth/my-account/api)).

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
