### Using Authenticated APIs (DefaultApi)

**Importante:** Debes establecer tu clave API en el ApiClient antes de hacer solicitudes autenticadas. Si no lo haces, las solicitudes fallarán con un error 401.

```ruby
require 'fastcomments'

# Crear y configurar el cliente API
config = FastCommentsClient::Configuration.new
api_client = FastCommentsClient::ApiClient.new(config)

# REQUERIDO: Establece tu clave API (obtén esto desde el panel de FastComments)
config.api_key['x-api-key'] = 'YOUR_API_KEY_HERE'

# Crear la instancia API con el cliente configurado
api = FastCommentsClient::DefaultApi.new(api_client)

# Ahora puedes hacer llamadas API autenticadas
begin
  # Ejemplo: Añadir un usuario SSO
  user_data = {
    id: 'user-123',
    email: 'user@example.com',
    displayName: 'John Doe'
  }

  response = api.add_sso_user('YOUR_TENANT_ID', user_data)
  puts "User created: #{response}"

rescue FastCommentsClient::ApiError => e
  puts "Error: #{e.response_body}"
  # Errores comunes:
  # - 401: Falta la clave API o es inválida
  # - 400: Falló la validación de la solicitud
end
```

### Using Public APIs (PublicApi)

Los endpoints públicos no requieren autenticación:

```ruby
require 'fastcomments'

public_api = FastCommentsClient::PublicApi.new

begin
  response = public_api.get_comments_public(
    'YOUR_TENANT_ID',
    'page-url-id'
  )
  puts response
rescue FastCommentsClient::ApiError => e
  puts e.message
end
```

### Using Moderation APIs (ModerationApi)

Los métodos de moderación impulsan el panel de moderador. Pasa un token `sso` para que la solicitud se haga en nombre de un moderador autenticado mediante SSO:

```ruby
require 'fastcomments'

moderation_api = FastCommentsClient::ModerationApi.new

begin
  # Ejemplo: Listar comentarios en la cola de moderación
  response = moderation_api.get_api_comments(
    sso: 'YOUR_MODERATOR_SSO_TOKEN'
  )
  puts response
rescue FastCommentsClient::ApiError => e
  puts e.message
end
```

### Common Issues

1. **Error 401 "missing-api-key"**: Asegúrate de establecer `config.api_key['x-api-key'] = 'YOUR_KEY'` antes de crear la instancia DefaultApi.
2. **Clase API incorrecta**: Usa `DefaultApi` para solicitudes autenticadas del lado del servidor, `PublicApi` para solicitudes del lado del cliente/públicas, y `ModerationApi` para solicitudes del panel de moderador.
3. **Clave API nula**: El SDK omitirá silenciosamente la autenticación si la clave API es nula, lo que provocará errores 401.