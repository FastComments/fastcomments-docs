## Παράμετροι

| Όνομα | Τύπος | Τοποθεσία | Απαραίτητο | Περιγραφή |
|------|------|----------|----------|-------------|
| value | string | query | Όχι |  |
| sso | string | query | Όχι |  |

## Απόκριση

Επιστρέφει: [`ModerationSiteSearchResponse`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/moderation_site_search_response.rb)

## Παράδειγμα

[inline-code-attrs-start title = 'get_search_sites Παράδειγμα'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'

api_instance = FastCommentsClient::ModerationApi.new
opts = {
  value: 'value_example', # Συμβολοσειρά | 
  sso: 'sso_example' # Συμβολοσειρά | 
}

begin
  
  result = api_instance.get_search_sites(opts)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling ModerationApi->get_search_sites: #{e}"
end
[inline-code-end]