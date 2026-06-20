## Parameter

| Name | Typ | Ort | Erforderlich | Beschreibung |
|------|------|----------|----------|-------------|
| tenantId | string | Pfad | Ja |  |
| locale | string | Query-Parameter | Nein |  |
| rating | string | Query-Parameter | Nein |  |
| page | number | Query-Parameter | Nein |  |

## Antwort

Gibt zurück: [`GetGifsTrendingResponse`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/get_gifs_trending_response.rb)

## Beispiel

[inline-code-attrs-start title = 'get_gifs_trending Beispiel'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'

api_instance = FastCommentsClient::PublicApi.new
tenant_id = 'tenant_id_example' # String | 
opts = {
  locale: 'locale_example', # String | 
  rating: 'rating_example', # String | 
  page: 1.2 # Float | 
}

begin
  
  result = api_instance.get_gifs_trending(tenant_id, opts)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling PublicApi->get_gifs_trending: #{e}"
end
[inline-code-end]