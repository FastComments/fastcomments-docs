## Parámetros

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes |  |
| id | string | path | Yes |  |

## Respuesta

Devuelve: [`GetTenantPackage200Response`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/get_tenant_package200_response.rb)

## Ejemplo

[inline-code-attrs-start title = 'Ejemplo de get_tenant_package'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'
# configurar autorización
FastCommentsClient.configure do |config|
  # Configurar la autorización de la clave API: api_key
  config.api_key['x-api-key'] = 'YOUR API KEY'
  # Descomente la siguiente línea para establecer un prefijo para la clave API, p. ej. 'Bearer' (por defecto nil)
  # config.api_key_prefix['x-api-key'] = 'Bearer'
end

api_instance = FastCommentsClient::DefaultApi.new
tenant_id = 'tenant_id_example' # String | 
id = 'id_example' # String | 

begin
  
  result = api_instance.get_tenant_package(tenant_id, id)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling DefaultApi->get_tenant_package: #{e}"
end
[inline-code-end]