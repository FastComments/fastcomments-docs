The plugin settings page is at **Site Administration > Plugins > Local plugins > FastComments**. The available options are:

#### Tenant ID

Your FastComments Tenant ID. Find this in the <a href="https://fastcomments.com/auth/my-account" target="_blank">FastComments dashboard</a> under your account settings.

#### API Secret

Your API Secret key, required for Secure SSO mode. Find this at <a href="https://fastcomments.com/auth/my-account/api-secret" target="_blank">My Account > API Secret</a>.

#### SSO Mode

Choose how users are authenticated. See the [SSO Modes](#items-moodle-sso-modes) section for details on each option.

- **Secure** (recommended) - server-side HMAC-SHA256 signed authentication
- **Simple** - client-side user data without signature
- **None** - anonymous commenting, no Moodle login integration

#### Page Contexts

Controls where comments appear:

- **Course pages** - comments on course main pages
- **Module/activity pages** - comments on individual activities and resources
- **Both** - comments on all page types

#### Commenting Style

Choose the commenting experience. See [Commenting Styles](#items-moodle-commenting-styles) for screenshots of each mode.

- **Comments** - standard threaded comment widget below page content
- **Collab Chat** - inline text selection discussions with presence indicators
- **Both** - comments and collab chat active together

#### CDN URL

The FastComments CDN URL. Defaults to `https://cdn.fastcomments.com`. Change this to the EU CDN URL if your data is hosted in the EU region.
