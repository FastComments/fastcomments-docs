Претходни коментатори на страници који НИСУ тренутно на мрежи. Сортирано по displayName.
Користите ово након што исцрпите /users/online да бисте приказали одељак "Чланови".
Курсорна пагинација по commenterName: сервер прелази парцијални {tenantId, urlId, commenterName}
индекс од afterName унапред користећи $gt, без трошка $skip.

## Параметри

| Име | Тип | Локација | Потребно | Опис |
|------|------|----------|----------|-------------|
| tenantId | string | path | Yes |  |
| urlId | string | query | Yes | Идентификатор URL странице (очишћен на серверу). |
| afterName | string | query | No | Курсор: проследите nextAfterName из претходног одговора. |
| afterUserId | string | query | No | Курсор: проследите nextAfterUserId из претходног одговора као разрешење једнакости. Потребно када је afterName подешен да именске везе не би испустиле уносе. |

## Одговор

Враћа: [`PageUsersOfflineResponse`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/page_users_offline_response.rb)

## Пример

[inline-code-attrs-start title = 'get_offline_users Пример'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'

api_instance = FastCommentsClient::PublicApi.new
tenant_id = 'tenant_id_example' # String | 
url_id = 'url_id_example' # String | Page URL identifier (cleaned server-side).
opts = {
  after_name: 'after_name_example', # String | Курсор: проследите nextAfterName из претходног одговора.
  after_user_id: 'after_user_id_example' # String | Курсор: проследите nextAfterUserId из претходног одговора као разрешење једнакости. Потребно када је afterName подешен да именске везе не би испустиле уносе.
}

begin
  
  result = api_instance.get_offline_users(tenant_id, url_id, opts)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling PublicApi->get_offline_users: #{e}"
end
[inline-code-end]