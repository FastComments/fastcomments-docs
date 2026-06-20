Прошли коментатори на страници који тренутно нису на мрежи. Сортирано по displayName.
Користите ово након исцрпљивања /users/online да бисте приказали "Чланови" секцију.
Курсорна пагинација по commenterName: сервер пролази делимични {tenantId, urlId, commenterName}
индекс почевши од afterName унапред помоћу $gt, без трошкова $skip.

## Параметри

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Yes |  |
| urlId | string | query | Yes | Идентификатор URL странице (очиšћен на серверу). |
| afterName | string | query | No | Курсор: проследите nextAfterName из претходног одговора. |
| afterUserId | string | query | No | Курсор за разлучавање истих имена: проследите nextAfterUserId из претходног одговора. Обавезно када је afterName подешен да не би дошло до губитка уноса због истих имена. |

## Одговор

Враћа: [`PageUsersOfflineResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/page_users_offline_response.py)

## Пример

[inline-code-attrs-start title = 'get_offline_users Пример'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.page_users_offline_response import PageUsersOfflineResponse
from client.rest import ApiException
from pprint import pprint

# Дефинисање host-а је опционално и подразумева се https://fastcomments.com
# Погледајте configuration.py за листу свих подржаних конфигурационих параметара.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# Уђите у контекст са инстанцом API клијента
with client.ApiClient(configuration) as api_client:
    # Креирајте инстанцу API класе
    api_instance = client.PublicApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    url_id = 'url_id_example' # str | Идентификатор URL странице (очишћен на серверу).
    after_name = 'after_name_example' # str | Курсор: проследите nextAfterName из претходног одговора. (опционално)
    after_user_id = 'after_user_id_example' # str | Курсор за разлучавање истих имена: проследите nextAfterUserId из претходног одговора. Обавезно када је afterName подешен да не би дошло до губитка уноса због истих имена. (опционално)

    try:
        api_response = api_instance.get_offline_users(tenant_id, url_id, after_name=after_name, after_user_id=after_user_id)
        print("The response of PublicApi->get_offline_users:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling PublicApi->get_offline_users: %s\n" % e)
[inline-code-end]