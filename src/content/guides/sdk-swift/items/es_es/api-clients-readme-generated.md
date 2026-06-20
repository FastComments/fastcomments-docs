El SDK de FastComments proporciona tres clientes de API:

### PublicAPI - Métodos seguros del lado del cliente

The `PublicAPI` contains methods that are safe to call from client-side code (iOS/macOS apps). These methods:
- No requieren una clave API
- Pueden usar tokens SSO para la autenticación
- Están limitados por tasa por usuario/dispositivo
- Son adecuados para aplicaciones orientadas al usuario final

**Ejemplo de uso**: Recuperar y crear comentarios en tu aplicación iOS

### DefaultAPI - Métodos del lado del servidor

The `DefaultAPI` contains authenticated methods that require an API key. These methods:
- Requieren tu clave API de FastComments
- SOLO deben llamarse desde código del lado del servidor
- Proporcionan acceso completo a tus datos de FastComments
- Están limitados por tasa por tenant

**Ejemplo de uso**: Operaciones administrativas, exportación masiva de datos, gestión de usuarios

### ModerationAPI - Métodos del panel de moderación

The `ModerationAPI` contains methods that power the moderator dashboard. These methods cover:
- **Moderación de comentarios** - listar, contar, buscar, recuperar registros y exportar comentarios
- **Acciones de moderación** - eliminar/restaurar comentarios, marcar, establecer estado de revisión/spam/aprobación, gestionar votos y reabrir/cerrar hilos
- **Prohibiciones** - prohibir a un usuario de un comentario, deshacer prohibiciones, obtener resúmenes previos a la prohibición, comprobar el estado y preferencias de la prohibición, y leer el recuento de usuarios prohibidos
- **Insignias y confianza** - otorgar/quitar insignias, listar insignias manuales, obtener/establecer el factor de confianza de un usuario y leer el perfil interno de un usuario

Every `ModerationAPI` method accepts an `sso` parameter so moderators can be authenticated via SSO.

**Ejemplo de uso**: Construir una experiencia de moderación para los moderadores de tu comunidad

**IMPORTANTE**: Nunca expongas tu clave API en código del lado del cliente. Las claves API deben usarse únicamente del lado del servidor.