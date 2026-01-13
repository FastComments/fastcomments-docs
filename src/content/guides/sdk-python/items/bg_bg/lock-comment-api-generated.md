## Параметри

| Име | Тип | Местоположение | Задължително | Описание |
|------|------|----------|----------|-------------|
| tenantId | string | path | Да |  |
| commentId | string | path | Да |  |
| broadcastId | string | query | Да |  |
| sso | string | query | Не |  |

## Отговор

Връща: [`LockComment200Response`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/lock_comment200_response.py)

## Пример

[inline-code-attrs-start title = 'lock_comment Пример'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.lock_comment200_response import LockComment200Response
from client.rest import ApiException
from pprint import pprint

# Задаването на host е по избор и по подразбиране е https://fastcomments.com
# Вижте configuration.py за списък с всички поддържани параметри на конфигурацията.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# Влезте в контекст с инстанция на API клиента
with client.ApiClient(configuration) as api_client:
    # Създайте инстанция на API класа
    api_instance = client.PublicApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    comment_id = 'comment_id_example' # str | 
    broadcast_id = 'broadcast_id_example' # str | 
    sso = 'sso_example' # str |  (по избор)

    try:
        api_response = api_instance.lock_comment(tenant_id, comment_id, broadcast_id, sso=sso)
        print("The response of PublicApi->lock_comment:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling PublicApi->lock_comment: %s\n" % e)
[inline-code-end]

---