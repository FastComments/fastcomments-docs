## Параметри

| Име | Type | Location | Обавезно | Опис |
|------|------|----------|----------|-------------|
| tenantId | string | query | Да |  |
| commentIds | string | query | Да | Списак идентификатора коментара одвојених зарезом. |
| sso | string | query | Не |  |

## Одговор

Враћа: [`CheckedCommentsForBlocked200Response`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/checked_comments_for_blocked200_response.py)

## Пример

[inline-code-attrs-start title = 'Пример за checked_comments_for_blocked'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.checked_comments_for_blocked200_response import CheckedCommentsForBlocked200Response
from client.rest import ApiException
from pprint import pprint

# Дефинисање host-а је опционално и подразумева https://fastcomments.com
# Погледајте configuration.py за списак свих подржаних конфигурационих параметара.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# Уђите у контекст са инстанцом API клијента
with client.ApiClient(configuration) as api_client:
    # Креирајте инстанцу API класе
    api_instance = client.PublicApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    comment_ids = 'comment_ids_example' # str | Списак идентификатора коментара одвојених зарезом.
    sso = 'sso_example' # str |  (опционо)

    try:
        api_response = api_instance.checked_comments_for_blocked(tenant_id, comment_ids, sso=sso)
        print("The response of PublicApi->checked_comments_for_blocked:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling PublicApi->checked_comments_for_blocked: %s\n" % e)
[inline-code-end]