В данный момент онлайн зрители страницы: люди, чья websocket-сессия в данный момент подписана на страницу.
Возвращает anonCount + totalCount (подписчики комнаты в целом, включая анонимных зрителей, которых мы не перечисляем).

## Parameters

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Да |  |
| urlId | string | query | Да | Идентификатор URL страницы (очищается на стороне сервера). |
| afterName | string | query | Нет | Курсор: передайте nextAfterName из предыдущего ответа. |
| afterUserId | string | query | Нет | Тайбрейкер курсора: передайте nextAfterUserId из предыдущего ответа. Обязательно при задании afterName, чтобы записи с одинаковыми именами не терялись. |

## Response

Возвращает: [`PageUsersOnlineResponse`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/page_users_online_response.rb)

## Example

[inline-code-attrs-start title = 'Пример get_online_users'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'

api_instance = FastCommentsClient::PublicApi.new
tenant_id = 'tenant_id_example' # String | 
url_id = 'url_id_example' # String | Идентификатор URL страницы (очищается на стороне сервера).
opts = {
  after_name: 'after_name_example', # String | Курсор: передайте nextAfterName из предыдущего ответа.
  after_user_id: 'after_user_id_example' # String | Тайбрейкер курсора: передайте nextAfterUserId из предыдущего ответа. Обязательно при задании afterName, чтобы записи с одинаковыми именами не терялись.
}

begin
  
  result = api_instance.get_online_users(tenant_id, url_id, opts)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling PublicApi->get_online_users: #{e}"
end
[inline-code-end]

---