## Paramètres

| Nom | Type | Emplacement | Obligatoire | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Oui |  |
| page | number | query | Non |  |
| count | number | query | Non |  |
| text-search | string | query | Non |  |
| byIPFromComment | string | query | Non |  |
| filters | string | query | Non |  |
| searchFilters | string | query | Non |  |
| sorts | string | query | Non |  |
| demo | boolean | query | Non |  |
| sso | string | query | Non |  |

## Réponse

Retourne : [`ModerationAPIGetCommentsResponse`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/moderation_a_p_i_get_comments_response.rb)

## Exemple

[inline-code-attrs-start title = 'Exemple get_api_comments'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'

api_instance = FastCommentsClient::ModerationApi.new
tenant_id = 'tenant_id_example' # Chaîne | 
opts = {
  page: 1.2, # Flottant | 
  count: 1.2, # Flottant | 
  text_search: 'text_search_example', # Chaîne | 
  by_ip_from_comment: 'by_ip_from_comment_example', # Chaîne | 
  filters: 'filters_example', # Chaîne | 
  search_filters: 'search_filters_example', # Chaîne | 
  sorts: 'sorts_example', # Chaîne | 
  demo: true, # Booléen | 
  sso: 'sso_example' # Chaîne | 
}

begin
  
  result = api_instance.get_api_comments(tenant_id, opts)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling ModerationApi->get_api_comments: #{e}"
end
[inline-code-end]