---
## Параметры

| Имя | Тип | Расположение | Обязательно | Описание |
|------|------|----------|----------|-------------|
| tenantId | string | путь | Да |  |
| commentId | string | путь | Да |  |
| broadcastId | string | строка запроса | Да |  |
| sso | string | строка запроса | Нет |  |

## Ответ

Возвращает: [`LockComment200Response`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/lock_comment200_response.rb)

## Пример

[inline-code-attrs-start title = 'Пример lock_comment'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'

api_instance = FastCommentsClient::PublicApi.new
tenant_id = 'tenant_id_example' # Строка | 
comment_id = 'comment_id_example' # Строка | 
broadcast_id = 'broadcast_id_example' # Строка | 
opts = {
  sso: 'sso_example' # Строка | 
}

begin
  
  result = api_instance.lock_comment(tenant_id, comment_id, broadcast_id, opts)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling PublicApi->lock_comment: #{e}"
end
[inline-code-end]

---