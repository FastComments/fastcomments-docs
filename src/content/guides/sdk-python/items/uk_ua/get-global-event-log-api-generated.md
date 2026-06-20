req
tenantId
urlId
userIdWS

## Параметри

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Так |  |
| urlId | string | query | Так |  |
| userIdWS | string | query | Так |  |
| startTime | integer | query | Так |  |
| endTime | integer | query | Ні |  |

## Відповідь

Повертає: [`GetEventLogResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/get_event_log_response.py)

## Приклад

[inline-code-attrs-start title = 'get_global_event_log Приклад'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.get_event_log_response import GetEventLogResponse
from client.rest import ApiException
from pprint import pprint

# Визначення host необов'язкове і за замовчуванням встановлено на https://fastcomments.com
# Див. configuration.py для списку всіх підтримуваних параметрів конфігурації.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# Відкрийте контекст з екземпляром клієнта API
with client.ApiClient(configuration) as api_client:
    # Створіть екземпляр класу API
    api_instance = client.PublicApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    url_id = 'url_id_example' # str | 
    user_id_ws = 'user_id_ws_example' # str | 
    start_time = 56 # int | 
    end_time = 56 # int |  (необов'язково)

    try:
        api_response = api_instance.get_global_event_log(tenant_id, url_id, user_id_ws, start_time, end_time=end_time)
        print("The response of PublicApi->get_global_event_log:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling PublicApi->get_global_event_log: %s\n" % e)
[inline-code-end]