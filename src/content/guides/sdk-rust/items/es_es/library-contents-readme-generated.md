El FastComments Rust SDK consta de varios módulos:

- **Client Module** - Cliente API autogenerado para las APIs REST de FastComments
  - Definiciones de tipos completas para todos los modelos de la API
  - Endpoints tanto autenticados (`DefaultApi`) como públicos (`PublicApi`)
  - Compatibilidad completa con async/await usando tokio
  - Consulte [client/README.md](https://github.com/FastComments/fastcomments-rust/blob/main/client/README.md) para documentación detallada de la API

- **SSO Module** - Utilidades de Single Sign-On del lado del servidor
  - Generación segura de tokens para la autenticación de usuarios
  - Soporte para modos SSO tanto simples como seguros
  - Firma de tokens basada en HMAC-SHA256

- **Core Types** - Definiciones de tipos compartidos y utilidades
  - Modelos de comentarios y estructuras de metadatos
  - Configuraciones de usuario y tenant
  - Funciones auxiliares para operaciones comunes