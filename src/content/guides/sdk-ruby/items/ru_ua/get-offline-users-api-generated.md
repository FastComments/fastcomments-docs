Past commenters on the page who are NOT currently online. Sorted by displayName.
Use this after exhausting /users/online to render a "Members" section.
Cursor pagination on commenterName: server walks the partial {tenantId, urlId, commenterName}
index from afterName forward via $gt, no $skip cost.

## Параметры

| Имя | Тип | Расположение | Обязательное | Описание |
|------|------|----------|----------|-------------|
| tenantId | string | path | Да |  |
| urlId | string | query | Да | Идентификатор URL страницы (очищается на стороне сервера). |
| afterName | string | query | Нет | Курсор: передайте nextAfterName из предыдущего ответа. |
| afterUserId | string | query | Нет | Дополнительный курсор для разрешения ничьих: передайте nextAfterUserId из предыдущего ответа. Обязательно, когда задан afterName, чтобы записи с одинаковыми именами не пропадали. |

## Ответ

Возвращает: [`PageUsersOfflineResponse`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/page_users_offline_response.rb)

## Пример

[inline-code-attrs-start title = 'Пример get_offline_users'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'

api_instance = FastCommentsClient::PublicApi.new
tenant_id = 'tenant_id_example' # String | 
url_id = 'url_id_example' # String | Идентификатор URL страницы (очищается на стороне сервера).
opts = {
  after_name: 'after_name_example', # String | Курсор: передайте nextAfterName из предыдущего ответа.
  after_user_id: 'after_user_id_example' # String | Разрешение ничьих для курсора: передайте nextAfterUserId из предыдущего ответа. Обязательно, когда задан afterName, чтобы записи с одинаковыми именами не пропадали.
}

begin
  
  result = api_instance.get_offline_users(tenant_id, url_id, opts)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling PublicApi->get_offline_users: #{e}"
end
[inline-code-end]