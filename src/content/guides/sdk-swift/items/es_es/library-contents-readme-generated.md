---
El SDK de FastComments para Swift consta de varios módulos:

- **Client Module** - cliente para las APIs REST de FastComments
  - Definiciones completas de tipos para todos los modelos de la API
  - Métodos autenticados (`DefaultAPI`), públicos (`PublicAPI`) y de moderación (`ModerationAPI`)
  - Compatibilidad completa con async/await
  - Consulte [client/README.md](https://github.com/FastComments/fastcomments-swift/blob/main/client/README.md) para documentación detallada de la API

- **SSO Module** - Utilidades del lado del servidor para inicio de sesión único (SSO)
  - Generación segura de tokens para la autenticación de usuarios
  - Soporte tanto para modos SSO simples como seguros
  - Firma de tokens basada en HMAC-SHA256 utilizando CryptoKit
---