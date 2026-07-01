---
El SDK de FastComments proporciona tres clientes API:

### PublicAPI - Métodos seguros para el cliente

El `PublicAPI` contiene métodos que son seguros de llamar desde código del lado del cliente (aplicaciones iOS/macOS). Estos métodos:
- No requieren una clave API
- Pueden usar tokens SSO para autenticación
- Tienen limitación de velocidad por usuario/dispositivo
- Son adecuados para aplicaciones dirigidas al usuario final

**Caso de uso ejemplo**: Obtener y crear comentarios en tu aplicación iOS

### DefaultAPI - Métodos del lado del servidor

El `DefaultAPI` contiene métodos autenticados que requieren una clave API. Estos métodos:
- Requieren tu clave API de FastComments
- DEBEN ser llamados únicamente desde código del lado del servidor
- Proporcionan acceso completo a tus datos de FastComments
- Tienen limitación de velocidad por inquilino

**Caso de uso ejemplo**: Operaciones administrativas, exportación masiva de datos, gestión de usuarios

### ModerationAPI - Métodos del panel de moderación

El `ModerationAPI` ofrece un conjunto extenso de APIs de moderación en vivo y rápidas. Cada método `ModerationAPI` acepta un parámetro `sso` y puede autenticarse vía SSO o una cookie de sesión de FastComments.com.

**Caso de uso ejemplo**: Construir una experiencia de moderación para los moderadores de tu comunidad

**IMPORTANTE**: Nunca expongas tu clave API en código del lado del cliente. Las claves API solo deben usarse del lado del servidor.
---