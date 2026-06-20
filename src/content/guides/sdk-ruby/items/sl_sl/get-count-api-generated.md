## Parametri

| Ime | Tip | Lokacija | Obvezno | Opis |
|------|------|----------|----------|-------------|
| text-search | string | poizvedba | Ne |  |
| byIPFromComment | string | poizvedba | Ne |  |
| filter | string | poizvedba | Ne |  |
| searchFilters | string | poizvedba | Ne |  |
| demo | boolean | poizvedba | Ne |  |
| sso | string | poizvedba | Ne |  |

## Odgovor

Vrača: [`ModerationAPICountCommentsResponse`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/moderation_a_p_i_count_comments_response.rb)

## Primer

[inline-code-attrs-start title = 'Primer get_count'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'

api_instance = FastCommentsClient::ModerationApi.new
opts = {
  text_search: 'text_search_example', # Niz | 
  by_ip_from_comment: 'by_ip_from_comment_example', # Niz | 
  filter: 'filter_example', # Niz | 
  search_filters: 'search_filters_example', # Niz | 
  demo: true, # Boolova vrednost | 
  sso: 'sso_example' # Niz | 
}

begin
  
  result = api_instance.get_count(opts)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling ModerationApi->get_count: #{e}"
end
[inline-code-end]