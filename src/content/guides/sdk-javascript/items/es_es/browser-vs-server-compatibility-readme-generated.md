Este SDK utiliza **puntos de entrada duales** para garantizar la compatibilidad óptima y prevenir errores en tiempo de ejecución:

- **`fastcomments-sdk/browser`** - Versión segura para el navegador con `fetch` nativo
- **`fastcomments-sdk/server`** - Versión completa para Node.js con soporte SSO
- **`fastcomments-sdk`** (predeterminado) - Solo tipos, seguro de importar en cualquier lugar