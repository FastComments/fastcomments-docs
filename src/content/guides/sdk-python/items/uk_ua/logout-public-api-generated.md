## Response

Повертає: [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/api_empty_response.py)

## Example

[inline-code-attrs-start title = 'Приклад logout_public'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.api_empty_response import APIEmptyResponse
from client.rest import ApiException
from pprint import pprint

# Визначення хоста необов'язкове й за замовчуванням встановлено на https://fastcomments.com
# Перегляньте configuration.py, щоб побачити список усіх підтримуваних параметрів конфігурації.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# Увійдіть у контекст із екземпляром клієнта API
with client.ApiClient(configuration) as api_client:
    # Створіть екземпляр класу API
    api_instance = client.PublicApi(api_client)

    try:
        api_response = api_instance.logout_public()
        print("The response of PublicApi->logout_public:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling PublicApi->logout_public: %s\n" % e)
[inline-code-end]