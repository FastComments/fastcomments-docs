## Parámetros

| Nombre | Tipo | Ubicación | Obligatorio | Descripción |
|------|------|----------|----------|-------------|
| tenantId | string | path | Sí |  |
| urlId | string | query | Sí |  |

## Respuesta

Devuelve: [`GetV2PageReacts`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/get_v2_page_reacts.rb)

## Ejemplo

[inline-code-attrs-start title = 'Ejemplo get_v2_page_reacts'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'

api_instance = FastCommentsClient::PublicApi.new
tenant_id = 'tenant_id_example' # Cadena | 
url_id = 'url_id_example' # Cadena | 

begin
  
  result = api_instance.get_v2_page_reacts(tenant_id, url_id)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling PublicApi->get_v2_page_reacts: #{e}"
end
[inline-code-end]

---