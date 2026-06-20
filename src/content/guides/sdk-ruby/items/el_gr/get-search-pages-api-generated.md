## Παράμετροι

| Name | Τύπος | Τοποθεσία | Απαιτείται | Περιγραφή |
|------|------|----------|----------|-------------|
| value | string | query | Όχι |  |
| sso | string | query | Όχι |  |

## Απάντηση

Επιστρέφει: [`ModerationPageSearchResponse`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/moderation_page_search_response.rb)

## Παράδειγμα

[inline-code-attrs-start title = 'Παράδειγμα get_search_pages'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'

api_instance = FastCommentsClient::ModerationApi.new
opts = {
  value: 'value_example', # Συμβολοσειρά | 
  sso: 'sso_example' # Συμβολοσειρά | 
}

begin
  
  result = api_instance.get_search_pages(opts)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling ModerationApi->get_search_pages: #{e}"
end
[inline-code-end]