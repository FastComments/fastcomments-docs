## Параметри

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Так |  |
| search | string | query | Так |  |
| locale | string | query | Ні |  |
| rating | string | query | Ні |  |
| page | number | query | Ні |  |

## Відповідь

Повертає: [`GetGifsSearchResponse`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/get_gifs_search_response.rb)

## Приклад

[inline-code-attrs-start title = 'Приклад get_gifs_search'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'

api_instance = FastCommentsClient::PublicApi.new
tenant_id = 'tenant_id_example' # Рядок | 
search = 'search_example' # Рядок | 
opts = {
  locale: 'locale_example', # Рядок | 
  rating: 'rating_example', # Рядок | 
  page: 1.2 # Число з плаваючою комою | 
}

begin
  
  result = api_instance.get_gifs_search(tenant_id, search, opts)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling PublicApi->get_gifs_search: #{e}"
end
[inline-code-end]