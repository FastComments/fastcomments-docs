Тренутно-онлајн гледаоци странице: људи чија је websocket сесија тренутно претплаћена на страницу.
Враћа anonCount + totalCount (претплатници у оквиру собе, укључујући анонимне гледаоце које не набрајамо).

## Параметри

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Да |  |
| urlId | string | query | Да | Page URL identifier (cleaned server-side). |
| afterName | string | query | Не | Cursor: pass nextAfterName from the previous response. |
| afterUserId | string | query | Не | Cursor tiebreaker: pass nextAfterUserId from the previous response. Required when afterName is set so name-ties don't drop entries. |

## Одговор

Враћа: [`PageUsersOnlineResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/page_users_online_response.py)

## Примјер

[inline-code-attrs-start title = 'Primjer get_online_users'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.page_users_online_response import PageUsersOnlineResponse
from client.rest import ApiException
from pprint import pprint

# Подешавање host-а је опциони и подразумјевано је https://fastcomments.com
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
    after_name = 'after_name_example' # str | Курсор: прослиједите nextAfterName из претходног одговора. (опционо)
    after_user_id = 'after_user_id_example' # str | Курсор тј. разрјешење веза: прослиједите nextAfterUserId из претходног одговора. Обавезно када је afterName подешен да се при истим именима не би изгубили уноси. (опционо)

    try:
        api_response = api_instance.get_online_users(tenant_id, url_id, after_name=after_name, after_user_id=after_user_id)
        print("The response of PublicApi->get_online_users:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling PublicApi->get_online_users: %s\n" % e)
[inline-code-end]