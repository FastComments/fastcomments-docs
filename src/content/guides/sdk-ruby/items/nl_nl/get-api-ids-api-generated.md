## Parameters

| Naam | Type | Locatie | Vereist | Beschrijving |
|------|------|----------|----------|-------------|
| text-search | string | query | Nee |  |
| byIPFromComment | string | query | Nee |  |
| filters | string | query | Nee |  |
| searchFilters | string | query | Nee |  |
| afterId | string | query | Nee |  |
| demo | boolean | query | Nee |  |
| sso | string | query | Nee |  |

## Respons

Retourneert: [`ModerationAPIGetCommentIdsResponse`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/moderation_a_p_i_get_comment_ids_response.rb)

## Voorbeeld

[inline-code-attrs-start title = 'get_api_ids Voorbeeld'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'

api_instance = FastCommentsClient::ModerationApi.new
opts = {
  text_search: 'text_search_example', # String | 
  by_ip_from_comment: 'by_ip_from_comment_example', # String | 
  filters: 'filters_example', # String | 
  search_filters: 'search_filters_example', # String | 
  after_id: 'after_id_example', # String | 
  demo: true, # Boolean | 
  sso: 'sso_example' # String | 
}

begin
  
  result = api_instance.get_api_ids(opts)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling ModerationApi->get_api_ids: #{e}"
end
[inline-code-end]

---