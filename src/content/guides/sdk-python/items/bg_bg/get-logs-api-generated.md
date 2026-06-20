## Параметри

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| commentId | string | path | Да |  |
| sso | string | query | Не |  |

## Отговор

Връща: [`ModerationAPIGetLogsResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/moderation_api_get_logs_response.py)

## Пример

[inline-code-attrs-start title = 'Пример get_logs'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.moderation_api_get_logs_response import ModerationAPIGetLogsResponse
from client.rest import ApiException
from pprint import pprint

# Дефинирането на host е по избор и по подразбиране е https://fastcomments.com
# Вижте configuration.py за списък с всички поддържани параметри за конфигурация.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# Влезте в контекст с инстанция на API клиента
with client.ApiClient(configuration) as api_client:
    # Създайте инстанция на API класа
    api_instance = client.ModerationApi(api_client)
    comment_id = 'comment_id_example' # str | 
    sso = 'sso_example' # str |  (незадължително)

    try:
        api_response = api_instance.get_logs(comment_id, sso=sso)
        print("The response of ModerationApi->get_logs:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling ModerationApi->get_logs: %s\n" % e)
[inline-code-end]