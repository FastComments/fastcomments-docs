## Παράμετροι

| Όνομα | Τύπος | Τοποθεσία | Απαιτείται | Περιγραφή |
|------|------|----------|----------|-------------|
| text-search | string | query | Όχι |  |
| byIPFromComment | string | query | Όχι |  |
| filter | string | query | Όχι |  |
| searchFilters | string | query | Όχι |  |
| demo | boolean | query | Όχι |  |
| sso | string | query | Όχι |  |

## Απόκριση

Επιστρέφει: [`ModerationAPICountCommentsResponse`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/moderation_a_p_i_count_comments_response.rb)

## Παράδειγμα

[inline-code-attrs-start title = 'Παράδειγμα get_count'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'

api_instance = FastCommentsClient::ModerationApi.new
opts = {
  text_search: 'text_search_example', # Συμβολοσειρά | 
  by_ip_from_comment: 'by_ip_from_comment_example', # Συμβολοσειρά | 
  filter: 'filter_example', # Συμβολοσειρά | 
  search_filters: 'search_filters_example', # Συμβολοσειρά | 
  demo: true, # Λογική | 
  sso: 'sso_example' # Συμβολοσειρά | 
}

begin
  
  result = api_instance.get_count(opts)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling ModerationApi->get_count: #{e}"
end
[inline-code-end]