## Параметри

| Име | Тип | Местоположение | Задължително | Описание |
|------|------|----------|----------|-------------|
| tenantId | string | query | Да |  |
| commentId | string | path | Да |  |
| isFlagged | boolean | query | Да |  |
| sso | string | query | Не |  |

## Отговор

Връща: [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/api_empty_response.py)

## Пример

[inline-code-attrs-start title = 'flag_comment_public Пример'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.api_empty_response import APIEmptyResponse
from client.rest import ApiException
from pprint import pprint

# Дефинирането на хоста е по избор и по подразбиране е https://fastcomments.com
# Вижте configuration.py за списък с всички поддържани параметри за конфигурация.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# Влезте в контекст с екземпляр на API клиента
with client.ApiClient(configuration) as api_client:
    # Създайте екземпляр на API класа
    api_instance = client.PublicApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    comment_id = 'comment_id_example' # str | 
    is_flagged = True # bool | 
    sso = 'sso_example' # str |  (optional)

    try:
        api_response = api_instance.flag_comment_public(tenant_id, comment_id, is_flagged, sso=sso)
        print("The response of PublicApi->flag_comment_public:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling PublicApi->flag_comment_public: %s\n" % e)
[inline-code-end]