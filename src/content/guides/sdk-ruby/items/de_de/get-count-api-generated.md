## Parameter

| Name | Typ | Location | Erforderlich | Beschreibung |
|------|------|----------|----------|-------------|
| text-search | string | query | Nein |  |
| byIPFromComment | string | query | Nein |  |
| filter | string | query | Nein |  |
| searchFilters | string | query | Nein |  |
| demo | boolean | query | Nein |  |
| sso | string | query | Nein |  |

## Antwort

Gibt zurück: [`ModerationAPICountCommentsResponse`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/moderation_a_p_i_count_comments_response.rb)

## Beispiel

[inline-code-attrs-start title = 'get_count Beispiel'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'

api_instance = FastCommentsClient::ModerationApi.new
opts = {
  text_search: 'text_search_example', # Zeichenkette | 
  by_ip_from_comment: 'by_ip_from_comment_example', # Zeichenkette | 
  filter: 'filter_example', # Zeichenkette | 
  search_filters: 'search_filters_example', # Zeichenkette | 
  demo: true, # Boolescher Wert | 
  sso: 'sso_example' # Zeichenkette | 
}

begin
  
  result = api_instance.get_count(opts)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling ModerationApi->get_count: #{e}"
end
[inline-code-end]