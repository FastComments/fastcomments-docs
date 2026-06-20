## Parametreler

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Evet |  |
| search | string | query | Evet |  |
| locale | string | query | Hayır |  |
| rating | string | query | Hayır |  |
| page | number | query | Hayır |  |

## Yanıt

Döndürür: [`GetGifsSearchResponse`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/get_gifs_search_response.rb)

## Örnek

[inline-code-attrs-start title = 'get_gifs_search Örneği'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'

api_instance = FastCommentsClient::PublicApi.new
tenant_id = 'tenant_id_example' # String | 
search = 'search_example' # String | 
opts = {
  locale: 'locale_example', # String | 
  rating: 'rating_example', # String | 
  page: 1.2 # Float | 
}

begin
  
  result = api_instance.get_gifs_search(tenant_id, search, opts)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling PublicApi->get_gifs_search: #{e}"
end
[inline-code-end]