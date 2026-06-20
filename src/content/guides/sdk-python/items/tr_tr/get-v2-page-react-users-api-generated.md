## Parametreler

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Evet |  |
| urlId | string | query | Evet |  |
| id | string | query | Evet |  |

## Response

Döndürür: [`GetV2PageReactUsersResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/get_v2_page_react_users_response.py)

## Örnek

[inline-code-attrs-start title = 'get_v2_page_react_users Örneği'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.get_v2_page_react_users_response import GetV2PageReactUsersResponse
from client.rest import ApiException
from pprint import pprint

# Host tanımlaması isteğe bağlıdır ve varsayılan https://fastcomments.com'dur
# Tüm desteklenen yapılandırma parametrelerinin listesi için configuration.py dosyasına bakın.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# API istemcisinin bir örneğiyle bir bağlam açın
with client.ApiClient(configuration) as api_client:
    # API sınıfının bir örneğini oluşturun
    api_instance = client.PublicApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    url_id = 'url_id_example' # str | 
    id = 'id_example' # str | 

    try:
        api_response = api_instance.get_v2_page_react_users(tenant_id, url_id, id)
        print("The response of PublicApi->get_v2_page_react_users:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling PublicApi->get_v2_page_react_users: %s\n" % e)
[inline-code-end]