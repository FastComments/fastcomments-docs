## Paramètres

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| text-search | string | query | No |  |
| byIPFromComment | string | query | No |  |
| filters | string | query | No |  |
| searchFilters | string | query | No |  |
| sorts | string | query | No |  |
| sso | string | query | No |  |

## Réponse

Retourne : [`ModerationExportResponse`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/moderation_export_response.rb)

## Exemple

[inline-code-attrs-start title = 'Exemple de post_api_export'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'

api_instance = FastCommentsClient::ModerationApi.new
opts = {
  text_search: 'text_search_example', # Chaîne | 
  by_ip_from_comment: 'by_ip_from_comment_example', # Chaîne | 
  filters: 'filters_example', # Chaîne | 
  search_filters: 'search_filters_example', # Chaîne | 
  sorts: 'sorts_example', # Chaîne | 
  sso: 'sso_example' # Chaîne | 
}

begin
  
  result = api_instance.post_api_export(opts)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling ModerationApi->post_api_export: #{e}"
end
[inline-code-end]