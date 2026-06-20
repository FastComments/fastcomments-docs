## Parametry

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| page | number | query | Nie |  |
| count | number | query | Nie |  |
| text-search | string | query | Nie |  |
| byIPFromComment | string | query | Nie |  |
| filters | string | query | Nie |  |
| searchFilters | string | query | Nie |  |
| sorts | string | query | Nie |  |
| demo | boolean | query | Nie |  |
| sso | string | query | Nie |  |

## Odpowiedź

Zwraca: [`ModerationAPIGetCommentsResponse`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/moderation_a_p_i_get_comments_response.rb)

## Przykład

[inline-code-attrs-start title = 'Przykład get_api_comments'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'

api_instance = FastCommentsClient::ModerationApi.new
opts = {
  page: 1.2, # Float | 
  count: 1.2, # Float | 
  text_search: 'text_search_example', # String | 
  by_ip_from_comment: 'by_ip_from_comment_example', # String | 
  filters: 'filters_example', # String | 
  search_filters: 'search_filters_example', # String | 
  sorts: 'sorts_example', # String | 
  demo: true, # Boolean | 
  sso: 'sso_example' # String | 
}

begin
  
  result = api_instance.get_api_comments(opts)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling ModerationApi->get_api_comments: #{e}"
end
[inline-code-end]