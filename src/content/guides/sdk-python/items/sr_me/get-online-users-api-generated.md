Тренутно онлајн гледаоци странице: особе чија websocket сесија је претплаћена на страницу у овом тренутку.
Враћа anonCount + totalCount (претплатници у соби у целини, укључујући анонимне гледаоце које не набрајамо).

## Параметри

| Име | Тип | Локација | Обавезно | Опис |
|------|------|----------|----------|-------------|
| tenantId | string | path | Да |  |
| urlId | string | query | Да | Page URL identifier (cleaned server-side). |
| afterName | string | query | Не | Курсор: проследите nextAfterName из претходног одговора. |
| afterUserId | string | query | Не | Курсор (tiebreaker): проследите nextAfterUserId из претходног одговора. Обавезно када је afterName подешен како би уноси са истим именом не били испуштени. |

## Одговор

Враћа: [`PageUsersOnlineResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/page_users_online_response.py)

## Пример

[inline-code-attrs-start title = 'get_online_users Пример'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.page_users_online_response import PageUsersOnlineResponse
from client.rest import ApiException
from pprint import pprint

# Дефинисање host-а је опционално и подразумевано је https://fastcomments.com
# Погледајте configuration.py за листу свих подржаних конфигурационих параметара.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# Уђите у контекст са инстанцом API клијента
with client.ApiClient(configuration) as api_client:
    # Креирајте инстанцу API класе
    api_instance = client.PublicApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    url_id = 'url_id_example' # str | Page URL identifier (cleaned server-side).
    after_name = 'after_name_example' # str | Курсор: проследите nextAfterName из претходног одговора. (опционо)
    after_user_id = 'after_user_id_example' # str | Курсор (tiebreaker): проследите nextAfterUserId из претходног одговора. Обавезно када је afterName подешен како би уноси са истим именом не били испуштени. (опционо)

    try:
        api_response = api_instance.get_online_users(tenant_id, url_id, after_name=after_name, after_user_id=after_user_id)
        print("The response of PublicApi->get_online_users:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling PublicApi->get_online_users: %s\n" % e)
[inline-code-end]