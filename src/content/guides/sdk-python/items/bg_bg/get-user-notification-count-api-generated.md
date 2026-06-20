## Параметри

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Да |  |
| sso | string | query | Не |  |

## Отговор

Връща: [`GetUserNotificationCountResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/get_user_notification_count_response.py)

## Пример

[inline-code-attrs-start title = 'Пример get_user_notification_count'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.get_user_notification_count_response import GetUserNotificationCountResponse
from client.rest import ApiException
from pprint import pprint

# Дефинирането на хоста е по избор и по подразбиране е https://fastcomments.com
# Вижте configuration.py за списък със всички поддържани параметри на конфигурацията.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# Влезте в контекст с инстанция на API клиента
with client.ApiClient(configuration) as api_client:
    # Създайте инстанция на API класа
    api_instance = client.PublicApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    sso = 'sso_example' # str |  (не задължително)

    try:
        api_response = api_instance.get_user_notification_count(tenant_id, sso=sso)
        print("The response of PublicApi->get_user_notification_count:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling PublicApi->get_user_notification_count: %s\n" % e)
[inline-code-end]