## Параметри

| Име | Тип | Местоположение | Задължително | Описание |
|------|------|----------|----------|-------------|
| urlId | string | query | Yes |  |
| sso | string | query | No |  |

## Отговор

Връща: [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/api_empty_response.py)

## Пример

[inline-code-attrs-start title = 'Пример за put_reopen_thread'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.api_empty_response import APIEmptyResponse
from client.rest import ApiException
from pprint import pprint

# Задаването на host е по избор и по подразбиране е https://fastcomments.com
# Вижте configuration.py за списък на всички поддържани параметри за конфигурация.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# Влезте в контекст с инстанция на API клиента
with client.ApiClient(configuration) as api_client:
    # Създайте инстанция на API класа
    api_instance = client.ModerationApi(api_client)
    url_id = 'url_id_example' # str | 
    sso = 'sso_example' # str |  (по избор)

    try:
        api_response = api_instance.put_reopen_thread(url_id, sso=sso)
        print("The response of ModerationApi->put_reopen_thread:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling ModerationApi->put_reopen_thread: %s\n" % e)
[inline-code-end]