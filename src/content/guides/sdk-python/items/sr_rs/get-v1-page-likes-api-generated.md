## Параметри

| Име | Тип | Локација | Обавезно | Опис |
|------|------|----------|----------|-------------|
| tenantId | string | path | Yes |  |
| urlId | string | query | Yes |  |

## Одговор

Враћа: [`GetV1PageLikes`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/get_v1_page_likes.py)

## Пример

[inline-code-attrs-start title = 'get_v1_page_likes Пример'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.get_v1_page_likes import GetV1PageLikes
from client.rest import ApiException
from pprint import pprint

# Постављање host-а је опционално и подразумевано је https://fastcomments.com
# Погледајте configuration.py за списак свих подржаних параметара конфигурације.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# Уђите у контекст са инстанцом API клијента
with client.ApiClient(configuration) as api_client:
    # Креирајте инстанцу API класе
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