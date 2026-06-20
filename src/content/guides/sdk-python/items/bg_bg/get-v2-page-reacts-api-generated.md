## Параметри

| Име | Тип | Местоположение | Задължително | Описание |
|------|------|----------|----------|-------------|
| tenantId | string | path | Да |  |
| urlId | string | query | Да |  |

## Отговор

Връща: [`GetV2PageReacts`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/get_v2_page_reacts.py)

## Пример

[inline-code-attrs-start title = 'get_v2_page_reacts Пример'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.get_v2_page_reacts import GetV2PageReacts
from client.rest import ApiException
from pprint import pprint

# Дефинирането на хоста е по избор и по подразбиране е https://fastcomments.com
# Вижте configuration.py за списък на всички поддържани параметри за конфигурация.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# Използвайте контекст с екземпляр на API клиента
with client.ApiClient(configuration) as api_client:
    # Създайте екземпляр на класа API
    api_instance = client.PublicApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    url_id = 'url_id_example' # str | 

    try:
        api_response = api_instance.get_v2_page_reacts(tenant_id, url_id)
        print("The response of PublicApi->get_v2_page_reacts:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling PublicApi->get_v2_page_reacts: %s\n" % e)
[inline-code-end]