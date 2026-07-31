All settings live in the plugin, under `System > Manage > Plugins > FastComments`.

| Setting | What it does |
|---|---|
| **Tenant ID** | Your FastComments Tenant ID, from [Settings > API/SSO](https://fastcomments.com/auth/my-account/api). Required. |
| **API Secret** | Your FastComments API Secret. Required only for Secure SSO. |
| **SSO Mode** | `Secure`, `Simple`, or `None`. See the Single Sign-On section. |
| **Commenting Style** | Choose Comments, Collab Chat, Live Chat, Image Chat, or a combination. See the Commenting Styles section. |
| **Show Comment Counts in Listings** | Shows a comment count next to each article title in category, blog, and featured list views. See the Comment Counts section. |
| **CDN URL** | Defaults to `https://cdn.fastcomments.com`. Change it for EU data residency. |
| **Site URL** | Defaults to `https://fastcomments.com`. Change it for EU data residency. Used for the no-JavaScript fallback. |

The widget uses your article's numeric ID for its thread identifier, so comment threads stay stable even if you change an article's title or alias.

<sup>Note!</sup> The comment widget renders on the single-article view only. Category, blog, and featured list views show a comment count (when enabled), not the full widget.
