### Errores 401 No autorizados

Si estás recibiendo errores 401 al usar la API autenticada:

1. **Comprueba tu API key**: Asegúrate de que estás usando la API key correcta en el panel de control de FastComments
2. **Verifica el tenant ID**: Asegúrate de que el tenant ID coincide con tu cuenta
3. **Formato de la API key**: La API key debe establecerse en el cliente de la API:

```swift
let defaultApi = DefaultAPI()
defaultApi.apiKey = "YOUR_API_KEY"
```

4. **Usando la API equivocada**: Asegúrate de usar `DefaultAPI` (no `PublicAPI`) para llamadas autenticadas

### Problemas con tokens SSO

Si los tokens SSO no funcionan:

1. **Usa el modo seguro en producción**: Siempre usa `FastCommentsSSO.createSecure()` con tu API key para producción
2. **Solo del lado del servidor**: Genera tokens SSO seguros en tu servidor, nunca expongas tu API key a los clientes
3. **Verifica los datos del usuario**: Asegúrate de que se proporcionan todos los campos obligatorios (id, email, username)
4. **Expiración del token**: Los tokens SSO seguros incluyen una marca de tiempo y pueden expirar. Genera tokens nuevos según sea necesario.

### Errores SSL/TLS

Si encuentras errores SSL/TLS:

1. Asegúrate de que el Info.plist de tu app permita conexiones HTTPS a fastcomments.com
2. Verifica que no estés usando excepciones de App Transport Security que puedan bloquear la conexión