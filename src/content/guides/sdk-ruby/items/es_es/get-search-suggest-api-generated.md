## Parámetros

| Nombre | Tipo | Ubicación | Obligatorio | Descripción |
|--------|------|-----------|-------------|-------------|
| tenantId | string | query | Sí |  |
| text-search | string | query | No |  |
| sso | string | query | No |  |

## Respuesta

Devuelve: [`ModerationSuggestResponse`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/moderation_suggest_response.rb)

## Ejemplo

[inline-code-attrs-start title = 'Ejemplo de get_search_suggest'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'

api_instance = FastCommentsClient::ModerationApi.new
tenant_id = 'tenant_id_example' # Cadena | 
opts = {
  text_search: 'text_search_example', # Cadena | 
  sso: 'sso_example' # Cadena | 
}

begin
  
  result = api_instance.get_search_suggest(tenant_id, opts)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling ModerationApi->get_search_suggest: #{e}"
end
[inline-code-end]

---