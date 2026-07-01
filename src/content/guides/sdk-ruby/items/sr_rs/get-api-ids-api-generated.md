## Параметри

| Име | Тип | Локација | Обавезно | Опис |
|------|------|----------|----------|------|
| tenantId | string | query | Да |  |
| text-search | string | query | Не |  |
| byIPFromComment | string | query | Не |  |
| filters | string | query | Не |  |
| searchFilters | string | query | Не |  |
| afterId | string | query | Не |  |
| demo | boolean | query | Не |  |
| sso | string | query | Не |  |

## Одговор

Враћа: [`ModerationAPIGetCommentIdsResponse`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/moderation_a_p_i_get_comment_ids_response.rb)

## Пример

[inline-code-attrs-start title = 'get_api_ids Primer'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'

api_instance = FastCommentsClient::ModerationApi.new
tenant_id = 'tenant_id_example' # Ниска | 
opts = {
  text_search: 'text_search_example', # Ниска | 
  by_ip_from_comment: 'by_ip_from_comment_example', # Ниска | 
  filters: 'filters_example', # Ниска | 
  search_filters: 'search_filters_example', # Ниска | 
  after_id: 'after_id_example', # Ниска | 
  demo: true, # Булеан | 
  sso: 'sso_example' # Ниска | 
}

begin
  
  result = api_instance.get_api_ids(tenant_id, opts)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling ModerationApi->get_api_ids: #{e}"
end
[inline-code-end]