---
Ta SDK uporablja **dvojne vstopne točke** za zagotavljanje optimalne združljivosti in preprečevanje napak med izvajanjem:

- **`fastcomments-sdk/browser`** - Različica varna za brskalnik z nativnim `fetch`
- **`fastcomments-sdk/server`** - Polna Node.js različica s podporo za SSO
- **`fastcomments-sdk`** (privzeto) - Samo tipi, varno za uvoz kjerkoli
---