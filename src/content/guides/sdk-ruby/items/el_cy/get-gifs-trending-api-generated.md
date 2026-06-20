## Παράμετροι

| Όνομα | Τύπος | Τοποθεσία | Απαιτείται | Περιγραφή |
|------|------|----------|----------|-------------|
| tenantId | string | path | Ναι |  |
| locale | string | query | Όχι |  |
| rating | string | query | Όχι |  |
| page | number | query | Όχι |  |

## Απόκριση

Επιστρέφει: [`GetGifsTrendingResponse`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/get_gifs_trending_response.rb)

## Παράδειγμα

[inline-code-attrs-start title = 'Παράδειγμα get_gifs_trending'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'

api_instance = FastCommentsClient::PublicApi.new
tenant_id = 'tenant_id_example' # Συμβολοσειρά | 
opts = {
  locale: 'locale_example', # Συμβολοσειρά | 
  rating: 'rating_example', # Συμβολοσειρά | 
  page: 1.2 # Δεκαδικός | 
}

begin
  
  result = api_instance.get_gifs_trending(tenant_id, opts)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling PublicApi->get_gifs_trending: #{e}"
end
[inline-code-end]