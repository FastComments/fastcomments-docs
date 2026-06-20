---
Предишни коментирали на страницата, които НЕ са в момента онлайн. Подредени по displayName.
Използвайте това след изчерпване на /users/online, за да рендерирате секция "Членове".
Пагинация с курсор върху commenterName: сървърът обхожда частичния индекс {tenantId, urlId, commenterName}
от afterName напред чрез $gt, без разход за $skip.

## Параметри

| Име | Тип | Местоположение | Задължително | Описание |
|------|------|----------|----------|-------------|
| tenantId | string | path | Yes |  |
| urlId | string | query | Yes | Идентификатор на URL на страницата (изчистен на сървъра). |
| afterName | string | query | No | Курсор: подайте nextAfterName от предишния отговор. |
| afterUserId | string | query | No | Разрешаване на равенства при курсора: подайте nextAfterUserId от предишния отговор. Задължително когато afterName е зададено, за да не се изпускат записи при еднакви имена. |

## Отговор

Връща: [`PageUsersOfflineResponse`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/page_users_offline_response.rb)

## Пример

[inline-code-attrs-start title = 'get_offline_users Пример'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'

api_instance = FastCommentsClient::PublicApi.new
tenant_id = 'tenant_id_example' # String | 
url_id = 'url_id_example' # String | Идентификатор на URL на страницата (изчистен на сървъра).
opts = {
  after_name: 'after_name_example', # String | Курсор: подайте nextAfterName от предишния отговор.
  after_user_id: 'after_user_id_example' # String | Разрешаване на равенства при курсора: подайте nextAfterUserId от предишния отговор. Задължително когато afterName е зададено, за да не се изпускат записи при еднакви имена.
}

begin
  
  result = api_instance.get_offline_users(tenant_id, url_id, opts)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling PublicApi->get_offline_users: #{e}"
end
[inline-code-end]

---