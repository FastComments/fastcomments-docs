## Параметри

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Так |  |
| urlId | string | query | Так |  |

## Відповідь

Повертає: [`GetV1PageLikes`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/get_v1_page_likes.py)

## Приклад

[inline-code-attrs-start title = 'Приклад get_v1_page_likes'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.get_v1_page_likes import GetV1PageLikes
from client.rest import ApiException
from pprint import pprint

# Визначення хоста необов'язкове та за замовчуванням https://fastcomments.com
# Див. configuration.py для списку всіх підтримуваних параметрів конфігурації.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# Увійдіть у контекст з екземпляром клієнта API
with client.ApiClient(configuration) as api_client:
    # Створіть екземпляр класу API
    api_instance = client.PublicApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    url_id = 'url_id_example' # str | 

    try:
        api_response = api_instance.get_v1_page_likes(tenant_id, url_id)
        print("The response of PublicApi->get_v1_page_likes:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling PublicApi->get_v1_page_likes: %s\n" % e)
[inline-code-end]