## Parámetros

| Nombre | Tipo | Ubicación | Obligatorio | Descripción |
|------|------|----------|----------|-------------|
| tenantId | string | query | Sí |  |
| id | string | path | Sí |  |
| editKey | string | query | No |  |

## Respuesta

Devuelve: [`VoteDeleteResponse`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/vote_delete_response.rb)

## Ejemplo

[inline-code-attrs-start title = 'Ejemplo de delete_vote'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'
# Configurar autorización
FastCommentsClient.configure do |config|
  # Configure la autorización por clave API: api_key
  config.api_key['x-api-key'] = 'YOUR API KEY'
  # Descomente la siguiente línea para establecer un prefijo para la clave API, p. ej. 'Bearer' (por defecto es nil)
  # config.api_key_prefix['x-api-key'] = 'Bearer'
end

api_instance = FastCommentsClient::DefaultApi.new
tenant_id = 'tenant_id_example' # String | 
id = 'id_example' # String | 
opts = {
  edit_key: 'edit_key_example' # String | 
}

begin
  
  result = api_instance.delete_vote(tenant_id, id, opts)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling DefaultApi->delete_vote: #{e}"
end
[inline-code-end]

---