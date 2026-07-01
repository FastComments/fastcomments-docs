## Parámetros

| Nombre | Tipo | Ubicación | Requerido | Descripción |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes |  |

## Respuesta

Devuelve: [`BulkCreateHashTagsResponse`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/bulk_create_hash_tags_response.rb)

## Ejemplo

[inline-code-attrs-start title = 'Ejemplo add_hash_tags_bulk'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'
# configurar autorización
FastCommentsClient.configure do |config|
  # Configurar autorización de clave API: api_key
  config.api_key['x-api-key'] = 'YOUR API KEY'
  # Descomente la línea siguiente para establecer un prefijo para la clave API, p.ej. 'Bearer' (por defecto nil)
  # config.api_key_prefix['x-api-key'] = 'Bearer'
end

api_instance = FastCommentsClient::DefaultApi.new
tenant_id = 'tenant_id_example' # String | 
bulk_create_hash_tags_body = FastCommentsClient::BulkCreateHashTagsBody.new({tags: [FastCommentsClient::BulkCreateHashTagsBodyTagsInner.new({tag: 'tag_example'})]}) # BulkCreateHashTagsBody | 

begin
  
  result = api_instance.add_hash_tags_bulk(tenant_id, bulk_create_hash_tags_body)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling DefaultApi->add_hash_tags_bulk: #{e}"
end
[inline-code-end]