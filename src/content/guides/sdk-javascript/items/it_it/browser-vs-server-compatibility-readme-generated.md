---
Questo SDK utilizza **due punti di ingresso** per garantire la massima compatibilità e prevenire errori a runtime:

- **`fastcomments-sdk/browser`** - Versione sicura per il browser con `fetch` nativo
- **`fastcomments-sdk/server`** - Versione completa per Node.js con supporto SSO
- **`fastcomments-sdk`** (predefinito) - Solo tipi, sicuro da importare ovunque
---