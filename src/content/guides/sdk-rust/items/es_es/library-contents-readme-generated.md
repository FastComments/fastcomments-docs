El SDK de FastComments para Rust consta de varios módulos:

- **Client Module** - cliente API para las APIs REST de FastComments
  - Definiciones de tipos completas para todos los modelos de la API
  - Tres clientes API que cubren todos los métodos de FastComments:
    - `default_api` (**DefaultApi**) - métodos autenticados con clave API para uso del lado del servidor
    - `public_api` (**PublicApi**) - métodos públicos, sin necesidad de clave API, seguros para llamar desde navegadores y aplicaciones móviles
    - `moderation_api` (**ModerationApi**) - un extenso conjunto de APIs de moderación en tiempo real y rápidas. Cada método de Moderación acepta un parámetro `sso` y puede autenticarse mediante SSO o una cookie de sesión de FastComments.com.
  - Soporte completo de async/await con tokio
  - Consulte [client/README.md](https://github.com/FastComments/fastcomments-rust/blob/main/client/README.md) para obtener documentación detallada de la API

- **SSO Module** - utilidades de Single Sign-On del lado del servidor
  - Generación segura de tokens para la autenticación de usuarios
  - Soporte para modos SSO simples y seguros
  - Firma de tokens basada en HMAC-SHA256

- **Core Types** - definiciones de tipos compartidas y utilidades
  - Modelos de comentarios y estructuras de metadatos
  - Configuraciones de usuario y arrendatario
  - Funciones auxiliares para operaciones comunes