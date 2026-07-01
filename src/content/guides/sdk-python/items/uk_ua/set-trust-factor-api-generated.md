## Параметри

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Так |  |
| userId | string | query | Ні |  |
| trustFactor | string | query | Ні |  |
| sso | string | query | Ні |  |

## Відповідь

Повертає: [`SetUserTrustFactorResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/set_user_trust_factor_response.py)

## Приклад

[inline-code-attrs-start title = 'set_trust_factor Приклад'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.api.moderation_api import SetTrustFactorOptions
from client.models.set_user_trust_factor_response import SetUserTrustFactorResponse
from client.rest import ApiException
from pprint import pprint

# Визначення хосту є необов’язковим і за замовчуванням дорівнює https://fastcomments.com
# Перегляньте configuration.py для отримання списку всіх підтримуваних параметрів конфігурації.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# Відкрити контекст з екземпляром API‑клієнта
with client.ApiClient(configuration) as api_client:
    # Створити екземпляр класу API
    api_instance = client.ModerationApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    user_id = 'user_id_example' # str |  (необов’язково)
    trust_factor = 'trust_factor_example' # str |  (необов’язково)
    sso = 'sso_example' # str |  (необов’язково)

    try:
        api_response = api_instance.set_trust_factor(tenant_id, SetTrustFactorOptions(user_id=user_id, trust_factor=trust_factor, sso=sso))
        print("The response of ModerationApi->set_trust_factor:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling ModerationApi->set_trust_factor: %s\n" % e)
[inline-code-end]