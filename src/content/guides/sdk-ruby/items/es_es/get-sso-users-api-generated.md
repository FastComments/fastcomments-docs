## Parámetros

| Nombre | Tipo | Ubicación | Requerido | Descripción |
|--------|------|----------|----------|-------------|
| tenantId | string | query | Sí |  |
| skip | integer | query | No |  |

## Respuesta

Devuelve: [`GetSSOUsersResponse`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/get_s_s_o_users_response.rb)

## Ejemplo

[inline-code-attrs-start title = 'Ejemplo get_sso_users'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'
# Configurar la autorización
FastCommentsClient.configure do |config|
  # Configurar la autorización por clave API: api_key
  config.api_key['x-api-key'] = 'YOUR API KEY'
  # Descomente la siguiente línea para establecer un prefijo para la clave API, p. ej. 'Bearer' (por defecto nil)
  # config.api_key_prefix['x-api-key'] = 'Bearer'
end

api_instance = FastCommentsClient::DefaultApi.new
tenant_id = 'tenant_id_example' # String | 
opts = {
  skip: 56 # Integer | 
}

begin
  
  result = api_instance.get_sso_users(tenant_id, opts)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling DefaultApi->get_sso_users: #{e}"
end
[inline-code-end]