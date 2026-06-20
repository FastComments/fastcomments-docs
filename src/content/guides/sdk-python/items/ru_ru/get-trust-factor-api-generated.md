## Параметры

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| userId | string | query | Нет |  |
| sso | string | query | Нет |  |

## Ответ

Возвращает: [`GetUserTrustFactorResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/get_user_trust_factor_response.py)

## Пример

[inline-code-attrs-start title = 'Пример get_trust_factor'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.get_user_trust_factor_response import GetUserTrustFactorResponse
from client.rest import ApiException
from pprint import pprint

# Указание host необязательно и по умолчанию равно https://fastcomments.com
# См. configuration.py для списка всех поддерживаемых параметров конфигурации.

configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# Входим в контекст с экземпляром клиента API
with client.ApiClient(configuration) as api_client:
    # Создаём экземпляр класса API
    api_instance = client.ModerationApi(api_client)
    user_id = 'user_id_example' # str | (необязательно)
    sso = 'sso_example' # str | (необязательно)

    try:
        api_response = api_instance.get_trust_factor(user_id=user_id, sso=sso)
        print("The response of ModerationApi->get_trust_factor:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling ModerationApi->get_trust_factor: %s\n" % e)
[inline-code-end]