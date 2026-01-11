---
El SDK de FastComments proporciona dos tipos de endpoints de API:

### PublicAPI - Client-Safe Endpoints

The `PublicAPI` contains endpoints that are safe to call from client-side code (iOS/macOS apps). These endpoints:
- No requieren una API key
- Pueden usar SSO tokens para la autenticación
- Están sujetas a límites de tasa por usuario/dispositivo
- Son adecuados para aplicaciones orientadas al usuario final

**Ejemplo de uso**: Recuperar y crear comentarios en tu app iOS

### DefaultAPI - Server-Side Endpoints

The `DefaultAPI` contains authenticated endpoints that require an API key. These endpoints:
- Requieren tu API key de FastComments
- DEBEN llamarse SÓLO desde código del lado del servidor
- Proporcionan acceso completo a tus datos de FastComments
- Están sujetas a límites de tasa por tenant

**Ejemplo de uso**: Operaciones administrativas, exportación masiva de datos, herramientas de moderación

**IMPORTANTE**: Nunca expongas tu API key en código del lado del cliente. Las API keys sólo deben usarse del lado del servidor.
---