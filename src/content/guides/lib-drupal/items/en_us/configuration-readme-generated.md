Navigate to **Administration > Configuration > Content > FastComments** (`/admin/config/content/fastcomments`).

### Settings

- **Tenant ID** (required) - Your FastComments Tenant ID. Find this under [Settings > API/SSO](https://fastcomments.com/auth/my-account/api) ([EU](https://eu.fastcomments.com/auth/my-account/api)).
- **API Secret** - Required for Secure SSO, webhook verification, and page sync. Found under [Settings > API/SSO](https://fastcomments.com/auth/my-account/api) ([EU](https://eu.fastcomments.com/auth/my-account/api)).
- **SSO Mode** - Single Sign-On integration:
  - **None** - No SSO, users comment as guests or create FastComments accounts.
  - **Simple** - Passes Drupal user info (name, email, avatar) to FastComments without server-side verification.
  - **Secure** - Uses HMAC-SHA256 verification to securely authenticate Drupal users with FastComments (recommended).
- **Commenting Style** - The type of widget to display:
  - **Live Comments** - Real-time threaded comments.
  - **Streaming Chat** - Live chat interface.
  - **Collab Chat** - Collaborative text-selection annotation on the main content area.
  - **Collab Chat + Comments** - Both collab chat and standard comments.
- **CDN URL** - FastComments CDN URL (default: `https://cdn.fastcomments.com`).
- **Site URL** - FastComments site URL (default: `https://fastcomments.com`).
- **Email notifications** - Send an email to content authors when a new comment is posted on their content.

### Adding Comments to Content Types

Add the **FastComments** field to your content types via **Structure > Content types > [type] > Manage fields**. The field has a status toggle and an optional custom identifier per entity.

### EU Data Residency

For EU data residency, update:
- **CDN URL** to `https://cdn-eu.fastcomments.com`
- **Site URL** to `https://eu.fastcomments.com`