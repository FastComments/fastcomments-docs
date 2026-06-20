Bivši komentatori na stranici koji trenutno НИСУ online. Sortirano po displayName.
Користите ово након што исцрпите /users/online да бисте приказали секцију "Чланови".
Курсорска пагинација по commenterName: сервер пролази парцијални {tenantId, urlId, commenterName}
индекс од afterName унапред преко $gt, без трошка $skip.

## Параметри

| Име | Тип | Локација | Обавезно | Опис |
|------|------|----------|----------|-------------|
| tenantId | string | path | Да |  |
| urlId | string | query | Да | Identifikator URL stranice (očišćen na serverskoj strani). |
| afterName | string | query | Не | Kursor: pošaljite nextAfterName iz prethodnog odgovora. |
| afterUserId | string | query | Не | Tiebreaker kursora: pošaljite nextAfterUserId iz prethodnog odgovora. Potrebno kada je afterName postavljen kako bi se unosi s istim imenom ne bi bili izgubljeni. |

## Одговор

Враћа: [`PageUsersOfflineResponse`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/page_users_offline_response.rb)

## Пример

[inline-code-attrs-start title = 'get_offline_users Пример'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'

api_instance = FastCommentsClient::PublicApi.new
tenant_id = 'tenant_id_example' # String | 
url_id = 'url_id_example' # String | Identifikator URL stranice (očišćen na serverskoj strani).
opts = {
  after_name: 'after_name_example', # String | Kursor: pošaljite nextAfterName iz prethodnog odgovora.
  after_user_id: 'after_user_id_example' # String | Tiebreaker kursora: pošaljite nextAfterUserId iz prethodnog odgovora. Potrebno kada je afterName postavljen kako bi se unosi s istim imenom ne bi bili izgubljeni.
}

begin
  
  result = api_instance.get_offline_users(tenant_id, url_id, opts)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling PublicApi->get_offline_users: #{e}"
end
[inline-code-end]