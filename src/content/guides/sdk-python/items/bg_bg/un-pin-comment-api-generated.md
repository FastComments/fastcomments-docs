## Параметри

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Да |  |
| commentId | string | path | Да |  |
| broadcastId | string | query | Да |  |
| sso | string | query | Не |  |

## Отговор

Връща: [`ChangeCommentPinStatusResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/change_comment_pin_status_response.py)

## Пример

[inline-code-attrs-start title = 'Пример за un_pin_comment'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.change_comment_pin_status_response import ChangeCommentPinStatusResponse
from client.rest import ApiException
from pprint import pprint

# Дефинирането на хоста е по избор и по подразбиране е https://fastcomments.com
# Вижте configuration.py за списък с всички поддържани конфигурационни параметри.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# Отворете контекст с екземпляр на API клиента
with client.ApiClient(configuration) as api_client:
    # Създайте екземпляр на класа API
    api_instance = client.PublicApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    comment_id = 'comment_id_example' # str | 
    broadcast_id = 'broadcast_id_example' # str | 
    sso = 'sso_example' # str |  (незадължително)

    try:
        api_response = api_instance.un_pin_comment(tenant_id, comment_id, broadcast_id, sso=sso)
        print("The response of PublicApi->un_pin_comment:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling PublicApi->un_pin_comment: %s\n" % e)
[inline-code-end]