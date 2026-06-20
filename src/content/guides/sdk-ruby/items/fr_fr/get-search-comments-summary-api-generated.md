## Paramètres

| Nom | Type | Emplacement | Requis | Description |
|------|------|----------|----------|-------------|
| value | string | query | Non |  |
| filters | string | query | Non |  |
| searchFilters | string | query | Non |  |
| sso | string | query | Non |  |

## Réponse

Renvoie : [`ModerationCommentSearchResponse`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/moderation_comment_search_response.rb)

## Exemple

[inline-code-attrs-start title = 'Exemple de get_search_comments_summary'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'

api_instance = FastCommentsClient::ModerationApi.new
opts = {
  value: 'value_example', # Chaîne | 
  filters: 'filters_example', # Chaîne | 
  search_filters: 'search_filters_example', # Chaîne | 
  sso: 'sso_example' # Chaîne | 
}

begin
  
  result = api_instance.get_search_comments_summary(opts)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling ModerationApi->get_search_comments_summary: #{e}"
end
[inline-code-end]