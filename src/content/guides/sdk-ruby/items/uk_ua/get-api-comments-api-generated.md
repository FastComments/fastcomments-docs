## Параметри

| Назва | Тип | Розташування | Обов’язковий | Опис |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes |  |
| page | number | query | No |  |
| count | number | query | No |  |
| text-search | string | query | No |  |
| byIPFromComment | string | query | No |  |
| filters | string | query | No |  |
| searchFilters | string | query | No |  |
| sorts | string | query | No |  |
| demo | boolean | query | No |  |
| sso | string | query | No |  |

## Відповідь

Повертає: [`ModerationAPIGetCommentsResponse`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/moderation_a_p_i_get_comments_response.rb)

## Приклад

[inline-code-attrs-start title = 'Приклад get_api_comments'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'

api_instance = FastCommentsClient::ModerationApi.new
tenant_id = 'tenant_id_example' # Рядок | 
opts = {
  page: 1.2, # Дробове | 
  count: 1.2, # Дробове | 
  text_search: 'text_search_example', # Рядок | 
  by_ip_from_comment: 'by_ip_from_comment_example', # Рядок | 
  filters: 'filters_example', # Рядок | 
  search_filters: 'search_filters_example', # Рядок | 
  sorts: 'sorts_example', # Рядок | 
  demo: true, # Логічний | 
  sso: 'sso_example' # Рядок | 
}

begin
  
  result = api_instance.get_api_comments(tenant_id, opts)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling ModerationApi->get_api_comments: #{e}"
end
[inline-code-end]

---