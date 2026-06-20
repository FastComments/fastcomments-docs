## Параметри

| Име | Тип | Местоположение | Задължителен | Описание |
|------|------|----------|----------|-------------|
| sso | string | query | Не |  |

## Отговор

Връща: [`GetTenantManualBadgesResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/get_tenant_manual_badges_response.py)

## Пример

[inline-code-attrs-start title = 'Пример get_manual_badges'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.get_tenant_manual_badges_response import GetTenantManualBadgesResponse
from client.rest import ApiException
from pprint import pprint

# Задаването на хост е по избор и по подразбиране е https://fastcomments.com
# Вижте configuration.py за списък с всички поддържани параметри на конфигурацията.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# Влезте в контекст с инстанция на API клиента
with client.ApiClient(configuration) as api_client:
    # Създайте инстанция на API класа
    api_instance = client.ModerationApi(api_client)
    sso = 'sso_example' # str | (по избор)

    try:
        api_response = api_instance.get_manual_badges(sso=sso)
        print("The response of ModerationApi->get_manual_badges:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling ModerationApi->get_manual_badges: %s\n" % e)
[inline-code-end]