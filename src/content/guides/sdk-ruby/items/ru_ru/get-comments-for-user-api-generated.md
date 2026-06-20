## Параметры

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| userId | string | query | Нет |  |
| direction | string | query | Нет |  |
| repliesToUserId | string | query | Нет |  |
| page | number | query | Нет |  |
| includei10n | boolean | query | Нет |  |
| locale | string | query | Нет |  |
| isCrawler | boolean | query | Нет |  |

## Ответ

Возвращает: [`GetCommentsForUserResponse`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/get_comments_for_user_response.rb)

## Пример

[inline-code-attrs-start title = 'Пример get_comments_for_user'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'

api_instance = FastCommentsClient::PublicApi.new
opts = {
  user_id: 'user_id_example', # String | 
  direction: FastCommentsClient::SortDirections::OF, # SortDirections | 
  replies_to_user_id: 'replies_to_user_id_example', # String | 
  page: 1.2, # Float | 
  includei10n: true, # Boolean | 
  locale: 'locale_example', # String | 
  is_crawler: true # Boolean | 
}

begin
  
  result = api_instance.get_comments_for_user(opts)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling PublicApi->get_comments_for_user: #{e}"
end
[inline-code-end]

---