Тренутно онлајн посетиоци странице: људи чија је websocket сесија претплаћена на страницу у овом тренутку.
Враћа anonCount + totalCount (сви претплатници собе, укључујући анонимне гледаоце које не набрајамо).

## Parameters

| Назив | Тип | Локација | Захтевано | Опис |
|------|------|----------|----------|-------------|
| tenantId | string | path | Yes |  |
| urlId | string | query | Yes | Идентификатор URL странице (очишћен на серверу). |
| afterName | string | query | No | Курсор: проследите nextAfterName из претходног одговора. |
| afterUserId | string | query | No | Решање везе за курсор: проследите nextAfterUserId из претходног одговора. Захтевано када је afterName постављено како би везе по имену не изоставиле уносе. |

## Response

Враћа: [`PageUsersOnlineResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/page_users_online_response.py)

## Example

[inline-code-attrs-start title = 'Пример get_online_users'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.page_users_online_response import PageUsersOnlineResponse
from client.rest import ApiException
from pprint import pprint

# Постављање host-а је опционално и подразумевано је https://fastcomments.com
# Погледајте configuration.py за списак свих подржаних конфигурационих параметара.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# Уђите у контекст са инстанцом API клијента
with client.ApiClient(configuration) as api_client:
    # Креирајте инстанцу API класе
    api_instance = client.PublicApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    url_id = 'url_id_example' # str | Идентификатор URL странице (очишћен на серверу).
    after_name = 'after_name_example' # str | Курсор: проследите nextAfterName из претходног одговора. (опционо)
    after_user_id = 'after_user_id_example' # str | Решање везе за курсор: проследите nextAfterUserId из претходног одговора. Захтевано када је afterName постављено како би везе по имену не изоставиле уносе. (опционо)

    try:
        api_response = api_instance.get_online_users(tenant_id, url_id, after_name=after_name, after_user_id=after_user_id)
        print("The response of PublicApi->get_online_users:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling PublicApi->get_online_users: %s\n" % e)
[inline-code-end]