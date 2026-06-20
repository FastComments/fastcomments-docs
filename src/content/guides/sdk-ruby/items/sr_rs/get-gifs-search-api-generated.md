## Параметри

| Име | Тип | Локација | Обавезно | Опис |
|------|------|----------|----------|-------------|
| tenantId | string | path | Да |  |
| search | string | query | Да |  |
| locale | string | query | Не |  |
| rating | string | query | Не |  |
| page | number | query | Не |  |

## Одговор

Враћа: [`GetGifsSearchResponse`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/get_gifs_search_response.rb)

## Пример

[inline-code-attrs-start title = 'get_gifs_search Пример'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
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

---