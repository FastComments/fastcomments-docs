## Paramètres

| Nom | Type | Emplacement | Obligatoire | Description |
|------|------|--------------|-------------|-------------|
| tenantId | string | query | Oui |  |
| text-search | string | query | Non |  |
| byIPFromComment | string | query | Non |  |
| filters | string | query | Non |  |
| searchFilters | string | query | Non |  |
| sorts | string | query | Non |  |
| sso | string | query | Non |  |

## Réponse

Renvoie : [`ModerationExportResponse`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/moderation_export_response.rb)

## Exemple

[inline-code-attrs-start title = 'post_api_export Exemple'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'

api_instance = FastCommentsClient::ModerationApi.new
tenant_id = 'tenant_id_example' # Chaîne | 
opts = {
  text_search: 'text_search_example', # Chaîne | 
  by_ip_from_comment: 'by_ip_from_comment_example', # Chaîne | 
  filters: 'filters_example', # Chaîne | 
  search_filters: 'search_filters_example', # Chaîne | 
  sorts: 'sorts_example', # Chaîne | 
  sso: 'sso_example' # Chaîne | 
}

begin
  
  result = api_instance.post_api_export(tenant_id, opts)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Erreur lors de l'appel de ModerationApi->post_api_export : #{e}"
end
[inline-code-end]