Тренутно онлајн посетиоци странице: људи чија вебсокет сесија је претплаћена на страницу у овом тренутку.  
Враћа anonCount + totalCount (претплатници у соби, укључујући анонимне посетиоце које не евидентирамо).

## Parameters

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Yes |  |
| urlId | string | query | Yes | Идентификатор URL странице (очишћен на серверу). |
| afterName | string | query | No | Курзор: проследите nextAfterName из претходног одговора. |
| afterUserId | string | query | No | Тијебрејкер за курзор: проследите nextAfterUserId из претходног одговора. Потребно када је afterName постављен како се не би изгубили уноси при везаним именима. |

## Response

Returns: [`PageUsersOnlineResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/page_users_online_response.py)

## Example

[inline-code-attrs-start title = 'Пример get_online_users'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.api.public_api import GetOnlineUsersOptions
from client.models.page_users_online_response import PageUsersOnlineResponse
from client.rest import ApiException
from pprint import pprint

# Дефинисање хоста је опционо и подразумевано је https://fastcomments.com
# Погледајте configuration.py за листу свих подржаних параметара конфигурације.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# Уђите у контекст са инстанцом API клијента
with client.ApiClient(configuration) as api_client:
    # Креирајте инстанцу API класе
    api_instance = client.PublicApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    url_id = 'url_id_example' # str | Идентификатор URL странице (очишћен на серверу).
    after_name = 'after_name_example' # str | Курзор: проследите nextAfterName из претходног одговора. (optional)
    after_user_id = 'after_user_id_example' # str | Тијебрејкер за курзор: проследите nextAfterUserId из претходног одговора. Потребно када је afterName постављен како се не би изгубили уноси при везаним именима. (optional)

    try:
        api_response = api_instance.get_online_users(tenant_id, url_id, GetOnlineUsersOptions(after_name=after_name, after_user_id=after_user_id))
        print("The response of PublicApi->get_online_users:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling PublicApi->get_online_users: %s\n" % e)
[inline-code-end]