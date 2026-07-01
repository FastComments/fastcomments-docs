## Παράμετροι

| Όνομα | Τύπος | Θέση | Απαιτείται | Περιγραφή |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes |  |
| text-search | string | query | No |  |
| byIPFromComment | string | query | No |  |
| filters | string | query | No |  |
| searchFilters | string | query | No |  |
| afterId | string | query | No |  |
| demo | boolean | query | No |  |
| sso | string | query | No |  |

## Απάντηση

Επιστρέφει: [`ModerationAPIGetCommentIdsResponse`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/moderation_a_p_i_get_comment_ids_response.rb)

## Παράδειγμα

[inline-code-attrs-start title = 'get_api_ids Παράδειγμα'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'

api_instance = FastCommentsClient::ModerationApi.new
tenant_id = 'tenant_id_example' # Συμβολοσειρά | 
opts = {
  text_search: 'text_search_example', # Συμβολοσειρά | 
  by_ip_from_comment: 'by_ip_from_comment_example', # Συμβολοσειρά | 
  filters: 'filters_example', # Συμβολοσειρά | 
  search_filters: 'search_filters_example', # Συμβολοσειρά | 
  after_id: 'after_id_example', # Συμβολοσειρά | 
  demo: true, # Boolean | 
  sso: 'sso_example' # Συμβολοσειρά | 
}

begin
  
  result = api_instance.get_api_ids(tenant_id, opts)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling ModerationApi->get_api_ids: #{e}"
end
[inline-code-end]