---
Dieses SDK verwendet **zwei Einstiegspunkte**, um optimale Kompatibilität sicherzustellen und Laufzeitfehler zu vermeiden:

- **`fastcomments-sdk/browser`** - Browser-sichere Version mit nativer `fetch`-Unterstützung
- **`fastcomments-sdk/server`** - Vollständige Node.js-Version mit SSO-Unterstützung
- **`fastcomments-sdk`** (Standard) - Nur Typen, sicher überall zu importieren
---