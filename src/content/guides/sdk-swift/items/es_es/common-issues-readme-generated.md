### 401 Errores No Autorizados

Si estás recibiendo errores 401 al usar la API autenticada:

1. **Verifica tu clave API**: Asegúrate de estar usando la clave API correcta de tu panel de FastComments  
2. **Verifica el ID del inquilino**: Asegúrate de que el ID del inquilino coincida con tu cuenta  
3. **Formato de la clave API**: La clave API debe establecerse como el encabezado `x-api-key` en la configuración compartida:

```swift
FastCommentsSwiftAPIConfiguration.shared.customHeaders["x-api-key"] = "YOUR_API_KEY"
```

4. **Usando la API incorrecta**: Asegúrate de estar usando `DefaultAPI` (no `PublicAPI`) para llamadas autenticadas  

### Problemas con Tokens SSO

Si los tokens SSO no funcionan:

1. **Usa modo seguro para producción**: Siempre usa `FastCommentsSSO.createSecure()` con tu clave API para producción  
2. **Solo del lado del servidor**: Genera tokens SSO seguros en tu servidor, nunca expongas tu clave API a los clientes  
3. **Verifica los datos del usuario**: Asegúrate de que todos los campos requeridos (id, email, nombre de usuario) se proporcionen  
4. **Expiración del token**: Los tokens SSO seguros incluyen una marca de tiempo y pueden expirar. Genera tokens nuevos según sea necesario.  

### Errores SSL/TLS

Si encuentras errores SSL/TLS:

1. Asegúrate de que el Info.plist de tu app permita conexiones HTTPS a fastcomments.com  
2. Verifica que no estés usando excepciones de App Transport Security que puedan bloquear la conexión