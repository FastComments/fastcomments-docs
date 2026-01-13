## Parámetros

| Nombre | Tipo | Ubicación | Requerido | Descripción |
|------|------|----------|----------|-------------|
| tenantId | string | query | Sí |  |
| meta | string | query | No |  |
| skip | number | query | No |  |

## Respuesta

Devuelve: [`GetTenants200Response`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/get_tenants200_response.rb)

## Ejemplo

[inline-code-attrs-start title = 'Ejemplo de get_tenants'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'
# configurar la autorización
FastCommentsClient.configure do |config|
  # Configurar la autorización de la clave de API: api_key
  config.api_key['x-api-key'] = 'YOUR API KEY'
  # Descomente la siguiente línea para establecer un prefijo para la clave de la API, p. ej. 'Bearer' (por defecto es nil)
  # config.api_key_prefix['x-api-key'] = 'Bearer'
end

api_instance = FastCommentsClient::DefaultApi.new
tenant_id = 'tenant_id_example' # String | 
opts = {
  meta: 'meta_example', # String | 
  skip: 1.2 # Float | 
}

begin
  
  result = api_instance.get_tenants(tenant_id, opts)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling DefaultApi->get_tenants: #{e}"
end
[inline-code-end]