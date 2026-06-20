Тренутно онлајн гледаоци странице: особе чија је websocket сесија тренутно претплаћена на ту страницу.
Враћа anonCount + totalCount (претплатници на нивоу собе, укључујући анонимне гледаоце које не набрајамо).

## Параметри

| Име | Тип | Локација | Обавезно | Опис |
|------|------|----------|----------|-------------|
| tenantId | string | path | Да |  |
| urlId | string | query | Да | Идентификатор URL странице (очишћен на серверској страни). |
| afterName | string | query | Не | Курсор: проследите nextAfterName из претходног одговора. |
| afterUserId | string | query | Не | Курсор за разрешење веза (tiebreaker): проследите nextAfterUserId из претходног одговора. Обавезно када је afterName постављен како би се у случају идентичних имена уноси не би изгубили. |

## Одговор

Враћа: [`PageUsersOnlineResponse`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/page_users_online_response.rb)

## Пример

[inline-code-attrs-start title = 'get_online_users Пример'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'

api_instance = FastCommentsClient::PublicApi.new
tenant_id = 'tenant_id_example' # String | 
url_id = 'url_id_example' # String | Идентификатор URL странице (очишћен на серверској страни).
opts = {
  after_name: 'after_name_example', # String | Курсор: проследите nextAfterName из претходног одговора.
  after_user_id: 'after_user_id_example' # String | Курсор за разрешење веза (tiebreaker): проследите nextAfterUserId из претходног одговора. Обавезно када је afterName постављен како би се у случају идентичних имена уноси не би изгубили.
}

begin
  
  result = api_instance.get_online_users(tenant_id, url_id, opts)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling PublicApi->get_online_users: #{e}"
end
[inline-code-end]