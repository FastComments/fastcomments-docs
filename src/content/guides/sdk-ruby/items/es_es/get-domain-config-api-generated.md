## Parámetros

| Nombre | Tipo | Ubicación | Obligatorio | Descripción |
|------|------|----------|----------|-------------|
| tenantId | string | query | Sí |  |
| domain | string | path | Sí |  |

## Respuesta

Devuelve: [`GetDomainConfigResponse`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/get_domain_config_response.rb)

## Ejemplo

[inline-code-attrs-start title = 'Ejemplo de get_domain_config'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'
# configurar autorización
FastCommentsClient.configure do |config|
  # Configurar la autorización de la clave API: api_key
  config.api_key['x-api-key'] = 'YOUR API KEY'
  # Descomenta la siguiente línea para establecer un prefijo para la clave API, p. ej. 'Bearer' (por defecto es nil)
  # config.api_key_prefix['x-api-key'] = 'Bearer'
end

api_instance = FastCommentsClient::DefaultApi.new
tenant_id = 'tenant_id_example' # String | 
domain = 'domain_example' # String | 

begin
  
  result = api_instance.get_domain_config(tenant_id, domain)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling DefaultApi->get_domain_config: #{e}"
end
[inline-code-end]

---