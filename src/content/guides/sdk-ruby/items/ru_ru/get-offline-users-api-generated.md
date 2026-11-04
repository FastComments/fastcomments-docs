Прошлые комментаторы страницы, которые НЕ находятся сейчас в сети. Отсортированы по displayName.
Используйте этот метод после исчерпания /users/online для отображения раздела «Участники».
Курсорная пагинация по commenterName: сервер проходит по частичному индексу {tenantId, urlId, commenterName}
от afterName вперёд через $gt, без затрат на $skip.

## Параметры

| Имя | Тип | Расположение | Обязательный | Описание |
|------|------|----------|----------|-------------|
| tenantId | string | path | Да |  |
| urlId | string | query | Да | Идентификатор URL страницы (очищается на сервере). |
| afterName | string | query | Нет | Курсор: передайте nextAfterName из предыдущего ответа. |
| afterUserId | string | query | Нет | Дополнительный ключ курсора: передайте nextAfterUserId из предыдущего ответа. Обязателен, когда задан afterName, чтобы записи с одинаковыми именами не терялись. |

## Ответ

Возвращает: [`PageUsersOfflineResponse`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/page_users_offline_response.rb)

## Пример

[inline-code-attrs-start title = 'Пример get_offline_users'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'

api_instance = FastCommentsClient::PublicApi.new
tenant_id = 'tenant_id_example' # String | 
url_id = 'url_id_example' # String | Идентификатор URL страницы (очищается на сервере).
opts = {
  after_name: 'after_name_example', # String | Курсор: передайте nextAfterName из предыдущего ответа.
  after_user_id: 'after_user_id_example' # String | Дополнительный ключ курсора: передайте nextAfterUserId из предыдущего ответа. Обязателен, когда задан afterName, чтобы записи с одинаковыми именами не терялись.
}

begin
  
  result = api_instance.get_offline_users(tenant_id, url_id, opts)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling PublicApi->get_offline_users: #{e}"
end
[inline-code-end]