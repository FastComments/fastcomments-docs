## Параметри

| Назва | Тип | Розташування | Обов'язкове | Опис |
|------|------|----------|----------|-------------|
| batchJobId | string | query | Ні |  |
| sso | string | query | Ні |  |

## Відповідь

Повертає: [`ModerationExportStatusResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/moderation_export_status_response.py)

## Приклад

[inline-code-attrs-start title = 'get_api_export_status Приклад'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.moderation_export_status_response import ModerationExportStatusResponse
from client.rest import ApiException
from pprint import pprint

# Визначення хоста необов'язкове й за замовчуванням встановлено на https://fastcomments.com
# Дивіться configuration.py для списку всіх підтримуваних параметрів конфігурації.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# Відкрийте контекст з екземпляром клієнта API
with client.ApiClient(configuration) as api_client:
    # Створіть екземпляр класу API
    api_instance = client.ModerationApi(api_client)
    batch_job_id = 'batch_job_id_example' # str |  (необов'язковий)
    sso = 'sso_example' # str |  (необов'язковий)

    try:
        api_response = api_instance.get_api_export_status(batch_job_id=batch_job_id, sso=sso)
        print("The response of ModerationApi->get_api_export_status:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling ModerationApi->get_api_export_status: %s\n" % e)
[inline-code-end]