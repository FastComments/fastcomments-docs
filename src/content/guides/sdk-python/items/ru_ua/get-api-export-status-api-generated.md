## Параметры

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| batchJobId | string | query | Нет |  |
| sso | string | query | Нет |  |

## Ответ

Возвращает: [`ModerationExportStatusResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/moderation_export_status_response.py)

## Пример

[inline-code-attrs-start title = 'Пример get_api_export_status'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.moderation_export_status_response import ModerationExportStatusResponse
from client.rest import ApiException
from pprint import pprint

# Defining the host is optional and defaults to https://fastcomments.com
# See configuration.py for a list of all supported configuration parameters.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# Enter a context with an instance of the API client
with client.ApiClient(configuration) as api_client:
    # Create an instance of the API class
    api_instance = client.ModerationApi(api_client)
    batch_job_id = 'batch_job_id_example' # str |  (необязательно)
    sso = 'sso_example' # str |  (необязательно)

    try:
        api_response = api_instance.get_api_export_status(batch_job_id=batch_job_id, sso=sso)
        print("The response of ModerationApi->get_api_export_status:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling ModerationApi->get_api_export_status: %s\n" % e)
[inline-code-end]

---