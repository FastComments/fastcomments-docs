## Параметри

| Име | Тип | Местоположение | Задължително | Описание |
|------|------|----------|----------|-------------|
| batchJobId | string | query | Не |  |
| sso | string | query | Не |  |

## Отговор

Връща: [`ModerationExportStatusResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/moderation_export_status_response.py)

## Пример

[inline-code-attrs-start title = 'Пример за get_api_export_status'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.moderation_export_status_response import ModerationExportStatusResponse
from client.rest import ApiException
from pprint import pprint

# Дефинирането на host е по избор и по подразбиране е https://fastcomments.com
# Вижте configuration.py за списък с всички поддържани параметри за конфигуриране.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# Влезте в контекст с инстанция на API клиента
with client.ApiClient(configuration) as api_client:
    # Създайте инстанция на класа API
    api_instance = client.ModerationApi(api_client)
    batch_job_id = 'batch_job_id_example' # str |  (по избор)
    sso = 'sso_example' # str |  (по избор)

    try:
        api_response = api_instance.get_api_export_status(batch_job_id=batch_job_id, sso=sso)
        print("The response of ModerationApi->get_api_export_status:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling ModerationApi->get_api_export_status: %s\n" % e)
[inline-code-end]