---
Пакетне информације о корисницима за tenant. За дате userIds, враћа информације за приказ из User / SSOUser.
Користи се у виџету коментара да обогати кориснике који су се управо појавили путем догађаја присутности.
Нема контекста странице: приватност се примењује уједначено (приватни профили су сакривени).

## Параметри

| Име | Тип | Локација | Обавезно | Опис |
|------|------|----------|----------|-------------|
| tenantId | string | path | Да |  |
| ids | string | query | Да | userIds раздвојени зарезом. |

## Одговор

Враћа: [`PageUsersInfoResponse`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/page_users_info_response.rb)

## Пример

[inline-code-attrs-start title = 'Пример get_users_info'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'

api_instance = FastCommentsClient::PublicApi.new
tenant_id = 'tenant_id_example' # String | 
ids = 'ids_example' # String | userIds раздвојени зарезом.

begin
  
  result = api_instance.get_users_info(tenant_id, ids)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling PublicApi->get_users_info: #{e}"
end
[inline-code-end]

---