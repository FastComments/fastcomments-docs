## Параметры

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| userId | string | query | Нет |  |
| trustFactor | string | query | Нет |  |
| sso | string | query | Нет |  |

## Ответ

Возвращает: [`SetUserTrustFactorResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/set_user_trust_factor_response.py)

## Пример

[inline-code-attrs-start title = 'Пример set_trust_factor'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.set_user_trust_factor_response import SetUserTrustFactorResponse
from client.rest import ApiException
from pprint import pprint

# Указание хоста необязательно и по умолчанию https://fastcomments.com
# См. configuration.py для списка всех поддерживаемых параметров конфигурации.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# Входим в контекст с экземпляром API-клиента
with client.ApiClient(configuration) as api_client:
    # Создаём экземпляр класса API
    api_instance = client.ModerationApi(api_client)
    user_id = 'user_id_example' # str |  (необязательно)
    trust_factor = 'trust_factor_example' # str |  (необязательно)
    sso = 'sso_example' # str |  (необязательно)

    try:
        api_response = api_instance.set_trust_factor(user_id=user_id, trust_factor=trust_factor, sso=sso)
        print("The response of ModerationApi->set_trust_factor:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling ModerationApi->set_trust_factor: %s\n" % e)
[inline-code-end]