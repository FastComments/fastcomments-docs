## Параметры

| Имя | Type | Location | Обязательно | Описание |
|------|------|----------|----------|-------------|
| tenantId | string | query | Да |  |
| commentIds | string | query | Да | Список идентификаторов комментариев, разделённых запятыми. |
| sso | string | query | Нет |  |

## Response

Возвращает: [`CheckBlockedCommentsResponse`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/check_blocked_comments_response.rb)

## Пример

[inline-code-attrs-start title = 'checked_comments_for_blocked Пример'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'

api_instance = FastCommentsClient::PublicApi.new
tenant_id = 'tenant_id_example' # String | 
comment_ids = 'comment_ids_example' # String | Список идентификаторов комментариев, разделённых запятыми.
opts = {
  sso: 'sso_example' # String | 
}

begin
  
  result = api_instance.checked_comments_for_blocked(tenant_id, comment_ids, opts)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling PublicApi->checked_comments_for_blocked: #{e}"
end
[inline-code-end]

---