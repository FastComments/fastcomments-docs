El SDK de FastComments para Rust consta de varios módulos:

- **Client Module** - Cliente API para las APIs REST de FastComments
  - Definiciones de tipo completas para todos los modelos de la API
  - Tres clientes de API que cubren todos los métodos de FastComments:
    - `default_api` (**DefaultApi**) - métodos autenticados con API-key para uso del lado del servidor
    - `public_api` (**PublicApi**) - métodos públicos, sin API-key, que son seguros para llamar desde navegadores y aplicaciones móviles
    - `moderation_api` (**ModerationApi**) - métodos que respaldan el panel de moderador, incluyendo moderación de comentarios (list, count, search, logs, export), acciones de moderación (remove/restore, flag, set review/spam/approval status, votes, reopen/close thread), bans (ban from a comment, undo, pre-ban summaries, ban status/preferences, banned-user counts), y badges & trust (award/remove badges, manual badges, get/set trust factor, user internal profile). Cada método de Moderation acepta un parámetro `sso` para que la llamada pueda hacerse en nombre de un moderador autenticado por SSO.
  - Soporte completo de async/await con tokio
  - Consulte [client/README.md](https://github.com/FastComments/fastcomments-rust/blob/main/client/README.md) para obtener documentación detallada de la API

- **SSO Module** - Utilidades de inicio de sesión único (SSO) del lado del servidor
  - Generación segura de tokens para autenticación de usuarios
  - Soporte para modos SSO simples y seguros
  - Firma de tokens basada en HMAC-SHA256

- **Core Types** - Definiciones de tipos compartidos y utilidades
  - Modelos de comentario y estructuras de metadatos
  - Configuraciones de usuario y tenant
  - Funciones auxiliares para operaciones comunes