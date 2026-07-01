## Параметри

| Име | Тип | Местоположение | Задължително | Описание |
|------|------|----------------|--------------|-----------|
| tenantId | string | query | Yes |  |
| userId | string | query | No |  |
| sso | string | query | No |  |

## Отговор

Връща: [`GetUserTrustFactorResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/get_user_trust_factor_response.py)

## Пример

[inline-code-attrs-start title = 'Пример за get_trust_factor'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.api.moderation_api import GetTrustFactorOptions
from client.models.get_user_trust_factor_response import GetUserTrustFactorResponse
from client.rest import ApiException
from pprint import pprint

# Дефинирането на хоста е по избор и по подразбиране е https://fastcomments.com
# Вижте configuration.py за списък с всички поддържани конфигурационни параметри.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# Въведете контекст с инстанция на API клиента
with client.ApiClient(configuration) as api_client:
    # Създаване на инстанция на API класа
    api_instance = client.ModerationApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    user_id = 'user_id_example' # str |  (по избор)
    sso = 'sso_example' # str |  (по избор)

    try:
        api_response = api_instance.get_trust_factor(tenant_id, GetTrustFactorOptions(user_id=user_id, sso=sso))
        print("The response of ModerationApi->get_trust_factor:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling ModerationApi->get_trust_factor: %s\n" % e)
[inline-code-end]