---
Користувачі, які наразі онлайн на сторінці: люди, чиї websocket сесії зараз підписані на цю сторінку.
Повертає anonCount + totalCount (підписники в межах кімнати, включаючи анонімних глядачів, яких ми не перелічуємо).

## Параметри

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Так |  |
| urlId | string | query | Так | Ідентифікатор URL сторінки (очищено на сервері). |
| afterName | string | query | Ні | Курсор: передайте nextAfterName з попередньої відповіді. |
| afterUserId | string | query | Ні | Розв'язувач нічиїх для курсора: передайте nextAfterUserId з попередньої відповіді. Потрібно, коли встановлено afterName, щоб записи з однаковими іменами не пропадали. |

## Відповідь

Повертає: [`PageUsersOnlineResponse`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/page_users_online_response.rb)

## Приклад

[inline-code-attrs-start title = 'Приклад get_online_users'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'

api_instance = FastCommentsClient::PublicApi.new
tenant_id = 'tenant_id_example' # String | 
url_id = 'url_id_example' # String | Ідентифікатор URL сторінки (очищено на сервері).
opts = {
  after_name: 'after_name_example', # String | Курсор: передайте nextAfterName з попередньої відповіді.
  after_user_id: 'after_user_id_example' # String | Розв'язувач нічиїх для курсора: передайте nextAfterUserId з попередньої відповіді. Потрібно, коли встановлено afterName, щоб записи з однаковими іменами не пропадали.
}

begin
  
  result = api_instance.get_online_users(tenant_id, url_id, opts)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling PublicApi->get_online_users: #{e}"
end
[inline-code-end]

---