El SDK de FastComments proporciona dos tipos de endpoints de API:

### PublicAPI - Endpoints Seguros para el Cliente

La `PublicAPI` contiene endpoints que son seguros para llamar desde código del lado del cliente (aplicaciones iOS/macOS). Estos endpoints:
- No requieren una clave de API
- Pueden usar tokens SSO para la autenticación
- Están limitados por tarifa por usuario/dispositivo
- Son adecuados para aplicaciones orientadas al usuario final

**Ejemplo de caso de uso**: Obtener y crear comentarios en tu aplicación iOS

### DefaultAPI - Endpoints del Lado del Servidor

La `DefaultAPI` contiene endpoints autenticados que requieren una clave de API. Estos endpoints:
- Requieren tu clave de API de FastComments
- Deben llamarse ÚNICAMENTE desde código del lado del servidor
- Proporcionan acceso completo a tus datos de FastComments
- Están limitados por tarifa por tenant

**Ejemplo de caso de uso**: Operaciones administrativas, exportación masiva de datos, herramientas de moderación

**IMPORTANTE**: Nunca expongas tu clave de API en código del lado del cliente. Las claves de API solo deben usarse del lado del servidor.