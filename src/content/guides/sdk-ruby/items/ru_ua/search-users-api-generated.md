## Параметры

| Имя | Type | Location | Обязательно | Описание |
|------|------|----------|----------|-------------|
| tenantId | string | path | Да |  |
| urlId | string | query | Да |  |
| usernameStartsWith | string | query | Нет |  |
| mentionGroupIds | array | query | Нет |  |
| sso | string | query | Нет |  |
| searchSection | string | query | Нет |  |

## Ответ

Возвращает: [`SearchUsers200Response`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/search_users200_response.rb)

## Пример

[inline-code-attrs-start title = 'Пример search_users'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'

api_instance = FastCommentsClient::PublicApi.new
tenant_id = 'tenant_id_example' # Строка | 
url_id = 'url_id_example' # Строка | 
opts = {
  username_starts_with: 'username_starts_with_example', # Строка | 
  mention_group_ids: ['inner_example'], # Массив<String> | 
  sso: 'sso_example', # Строка | 
  search_section: 'fast' # Строка | 
}

begin
  
  result = api_instance.search_users(tenant_id, url_id, opts)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling PublicApi->search_users: #{e}"
end
[inline-code-end]