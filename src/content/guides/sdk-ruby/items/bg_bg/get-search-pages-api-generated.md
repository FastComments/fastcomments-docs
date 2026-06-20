---
## Параметри

| Име | Тип | Местоположение | Задължително | Описание |
|------|------|----------|----------|-------------|
| value | string | query | Не |  |
| sso | string | query | Не |  |

## Отговор

Връща: [`ModerationPageSearchResponse`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/moderation_page_search_response.rb)

## Пример

[inline-code-attrs-start title = 'get_search_pages Пример'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'

api_instance = FastCommentsClient::ModerationApi.new
opts = {
  value: 'value_example', # Низ | 
  sso: 'sso_example' # Низ | 
}

begin
  
  result = api_instance.get_search_pages(opts)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling ModerationApi->get_search_pages: #{e}"
end
[inline-code-end]

---