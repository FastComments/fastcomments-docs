El SDK de FastComments para Swift consta de varios módulos:

- **Módulo del cliente** - Cliente de API generado automáticamente para las APIs REST de FastComments
  - Definiciones completas de tipos para todos los modelos de la API
  - Endpoints tanto autenticados (`DefaultAPI`) como públicos (`PublicAPI`)
  - Soporte completo para async/await
  - Consulte [client/README.md](https://github.com/FastComments/fastcomments-swift/blob/main/client/README.md) para obtener documentación detallada de la API

- **Módulo SSO** - Utilidades de inicio de sesión único (Single Sign-On) del lado del servidor
  - Generación segura de tokens para la autenticación de usuarios
  - Soporte tanto para modos SSO simples como seguros
  - Firmado de tokens basado en HMAC-SHA256 usando CryptoKit