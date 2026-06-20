## Параметри

| Назва | Тип | Розташування | Обов'язково | Опис |
|------|------|----------|----------|-------------|
| userId | string | query | Ні |  |
| sso | string | query | Ні |  |

## Відповідь

Повертає: [`GetUserTrustFactorResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/get_user_trust_factor_response.py)

## Приклад

[inline-code-attrs-start title = 'Приклад get_trust_factor'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.get_user_trust_factor_response import GetUserTrustFactorResponse
from client.rest import ApiException
from pprint import pprint

# Визначення хоста необов'язкове і за замовчуванням — https://fastcomments.com
# Див. configuration.py для списку всіх підтримуваних параметрів конфігурації.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# Увійдіть у контекст із екземпляром клієнта API
with client.ApiClient(configuration) as api_client:
    # Створіть екземпляр класу API
    api_instance = client.ModerationApi(api_client)
    user_id = 'user_id_example' # str |  (необов'язково)
    sso = 'sso_example' # str |  (необов'язково)

    try:
        api_response = api_instance.get_trust_factor(user_id=user_id, sso=sso)
        print("The response of ModerationApi->get_trust_factor:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling ModerationApi->get_trust_factor: %s\n" % e)
[inline-code-end]