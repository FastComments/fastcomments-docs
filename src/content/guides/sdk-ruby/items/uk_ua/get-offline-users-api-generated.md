Попередні коментатори на сторінці, які наразі НЕ в мережі. Відсортовано за displayName.
Використовуйте це після вичерпання /users/online, щоб відобразити секцію "Members".
Курсорна пагінація за commenterName: сервер обходить частковий індекс {tenantId, urlId, commenterName}
індекс від afterName вперед через $gt, без витрат на $skip.

## Параметри

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Yes |  |
| urlId | string | query | Yes | Ідентифікатор URL сторінки (очищується на сервері). |
| afterName | string | query | No | Курсор: передайте nextAfterName з попередньої відповіді. |
| afterUserId | string | query | No | Тайбрейкер курсора: передайте nextAfterUserId з попередньої відповіді. Обов'язково, коли встановлено afterName, щоб при однакових іменах записи не пропадали. |

## Відповідь

Повертає: [`PageUsersOfflineResponse`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/page_users_offline_response.rb)

## Приклад

[inline-code-attrs-start title = 'Приклад get_offline_users'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'

api_instance = FastCommentsClient::PublicApi.new
tenant_id = 'tenant_id_example' # String | 
url_id = 'url_id_example' # String | Ідентифікатор URL сторінки (очищується на сервері).
opts = {
  after_name: 'after_name_example', # String | Курсор: передайте nextAfterName з попередньої відповіді.
  after_user_id: 'after_user_id_example' # String | Тайбрейкер курсора: передайте nextAfterUserId з попередньої відповіді. Обов'язково, коли встановлено afterName, щоб при однакових іменах записи не пропадали.
}

begin
  
  result = api_instance.get_offline_users(tenant_id, url_id, opts)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling PublicApi->get_offline_users: #{e}"
end
[inline-code-end]

---