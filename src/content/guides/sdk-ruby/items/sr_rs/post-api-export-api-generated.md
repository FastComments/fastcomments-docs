## Параметри

| Име | Тип | Локација | Обавезно | Опис |
|------|------|----------|----------|-------------|
| text-search | string | query | Не |  |
| byIPFromComment | string | query | Не |  |
| filters | string | query | Не |  |
| searchFilters | string | query | Не |  |
| sorts | string | query | Не |  |
| sso | string | query | Не |  |

## Одговор

Враћа: [`ModerationExportResponse`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/moderation_export_response.rb)

## Пример

[inline-code-attrs-start title = 'post_api_export Пример'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'

api_instance = FastCommentsClient::ModerationApi.new
opts = {
  text_search: 'text_search_example', # Стринг | 
  by_ip_from_comment: 'by_ip_from_comment_example', # Стринг | 
  filters: 'filters_example', # Стринг | 
  search_filters: 'search_filters_example', # Стринг | 
  sorts: 'sorts_example', # Стринг | 
  sso: 'sso_example' # Стринг | 
}

begin
  
  result = api_instance.post_api_export(opts)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling ModerationApi->post_api_export: #{e}"
end
[inline-code-end]