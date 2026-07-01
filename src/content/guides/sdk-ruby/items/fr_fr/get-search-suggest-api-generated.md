## Paramètres

| Nom | Type | Emplacement | Requis | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Oui |  |
| text-search | string | query | Non |  |
| sso | string | query | Non |  |

## Réponse

Retourne : [`ModerationSuggestResponse`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/moderation_suggest_response.rb)

## Exemple

[inline-code-attrs-start title = 'Exemple de get_search_suggest'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'

api_instance = FastCommentsClient::ModerationApi.new
tenant_id = 'tenant_id_example' # String | 
opts = {
  text_search: 'text_search_example', # String | 
  sso: 'sso_example' # String | 
}

begin
  
  result = api_instance.get_search_suggest(tenant_id, opts)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Erreur lors de l'appel de ModerationApi->get_search_suggest : #{e}"
end
[inline-code-end]