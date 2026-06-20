Понастоящем онлайн зрители на страница: хора, чиито websocket сесии са абонирани за страницата в този момент.
Връща anonCount + totalCount (абонати на стаята, включително анонимни зрители, които не изброяваме).

## Параметри

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Yes |  |
| urlId | string | query | Yes | Идентификатор на URL на страницата (изчистен от сървъра). |
| afterName | string | query | No | Курсор: предайте nextAfterName от предишния отговор. |
| afterUserId | string | query | No | Курсор за разрешаване на равенства: предайте nextAfterUserId от предишния отговор. Задължително когато afterName е зададено, за да не се изпускат записи при еднакви имена. |

## Отговор

Връща: [`PageUsersOnlineResponse`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/page_users_online_response.rb)

## Пример

[inline-code-attrs-start title = 'Пример за get_online_users'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'

api_instance = FastCommentsClient::PublicApi.new
tenant_id = 'tenant_id_example' # String | 
url_id = 'url_id_example' # String | Идентификатор на URL на страницата (изчистен от сървъра).
opts = {
  after_name: 'after_name_example', # String | Курсор: предайте nextAfterName от предишния отговор.
  after_user_id: 'after_user_id_example' # String | Курсор за разрешаване на равенства: предайте nextAfterUserId от предишния отговор. Задължително когато afterName е зададено, за да не се изпускат записи при еднакви имена.
}

begin
  
  result = api_instance.get_online_users(tenant_id, url_id, opts)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling PublicApi->get_online_users: #{e}"
end
[inline-code-end]