This SDK uses **dual entry points** to ensure optimal compatibility and prevent runtime errors:

- **`fastcomments-sdk/browser`** - Browser-safe version with native `fetch`
- **`fastcomments-sdk/server`** - Full Node.js version with SSO support
- **`fastcomments-sdk`** (default) - Types only, safe to import anywhere