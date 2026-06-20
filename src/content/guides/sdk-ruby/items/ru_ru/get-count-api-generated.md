## Параметры

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| text-search | string | query | Нет |  |
| byIPFromComment | string | query | Нет |  |
| filter | string | query | Нет |  |
| searchFilters | string | query | Нет |  |
| demo | boolean | query | Нет |  |
| sso | string | query | Нет |  |

## Ответ

Возвращает: [`ModerationAPICountCommentsResponse`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/moderation_a_p_i_count_comments_response.rb)

## Пример

[inline-code-attrs-start title = 'Пример get_count'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'

api_instance = FastCommentsClient::ModerationApi.new
opts = {
  text_search: 'text_search_example', # String | 
  by_ip_from_comment: 'by_ip_from_comment_example', # String | 
  filter: 'filter_example', # String | 
  search_filters: 'search_filters_example', # String | 
  demo: true, # Boolean | 
  sso: 'sso_example' # String | 
}

begin
  
  result = api_instance.get_count(opts)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling ModerationApi->get_count: #{e}"
end
[inline-code-end]