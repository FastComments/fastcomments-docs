### Uso de APIs autenticadas (DefaultApi)

**Important:** Debes establecer tu clave de API en el ApiClient antes de realizar solicitudes autenticadas. Si no lo haces, las solicitudes fallarán con un error 401.

```ruby
require 'fastcomments-client'

# Crear y configurar el cliente de API
config = FastCommentsClient::Configuration.new
api_client = FastCommentsClient::ApiClient.new(config)

# OBLIGATORIO: Establece tu clave de API (obténla desde tu panel de FastComments)
config.api_key['x-api-key'] = 'YOUR_API_KEY_HERE'

# Crea la instancia de la API con el cliente configurado
api = FastCommentsClient::DefaultApi.new(api_client)

# Ahora puedes hacer llamadas a la API autenticadas
begin
  # Ejemplo: Agregar un usuario SSO
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
  # - 401: falta o es inválida la clave API
  # - 400: la validación de la solicitud falló
end
```

### Uso de APIs públicas (PublicApi)

Los endpoints públicos no requieren autenticación:

```ruby
require 'fastcomments-client'

public_api = FastCommentsClient::PublicApi.new

begin
  response = public_api.get_comments_public(
    tenant_id: 'YOUR_TENANT_ID',
    url_id: 'page-url-id'
  )
  puts response
rescue FastCommentsClient::ApiError => e
  puts e.message
end
```

### Problemas comunes

1. **401 "missing-api-key" error**: Asegúrate de establecer `config.api_key['x-api-key'] = 'YOUR_KEY'` antes de crear la instancia DefaultApi.
2. **Wrong API class**: Usa `DefaultApi` para solicitudes autenticadas del lado del servidor, `PublicApi` para solicitudes del lado del cliente/públicas.
3. **Null API key**: El SDK omitirá silenciosamente la autenticación si la clave API es null, lo que provocará errores 401.