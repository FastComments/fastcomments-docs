---
Este SDK usa **dois pontos de entrada** para garantir compatibilidade ideal e prevenir erros em tempo de execução:

- **`fastcomments-sdk/browser`** - Versão segura para navegador com `fetch` nativo
- **`fastcomments-sdk/server`** - Versão completa para Node.js com suporte a SSO
- **`fastcomments-sdk`** (padrão) - Apenas tipos, seguro para importar em qualquer lugar
---