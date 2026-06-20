## Параметри

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Да |  |
| commentIds | string | query | Да | Листа идентификатора коментара, одвојених зарезом. |
| sso | string | query | Не |  |

## Одговор

Враћа: [`CheckBlockedCommentsResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/check_blocked_comments_response.py)

## Пример

[inline-code-attrs-start title = 'Пример checked_comments_for_blocked'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.check_blocked_comments_response import CheckBlockedCommentsResponse
from client.rest import ApiException
from pprint import pprint

# Постављање host-а је опционално и подразумевано је https://fastcomments.com
# Погледајте configuration.py за листу свих подржаних параметара конфигурације.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# Уђите у контекст са инстанцом API клијента
with client.ApiClient(configuration) as api_client:
    # Креирајте инстанцу API класе
    api_instance = client.PublicApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    comment_ids = 'comment_ids_example' # str | Листа идентификатора коментара одвојених зарезом.
    sso = 'sso_example' # str |  (опционо)

    try:
        api_response = api_instance.checked_comments_for_blocked(tenant_id, comment_ids, sso=sso)
        print("The response of PublicApi->checked_comments_for_blocked:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling PublicApi->checked_comments_for_blocked: %s\n" % e)
[inline-code-end]