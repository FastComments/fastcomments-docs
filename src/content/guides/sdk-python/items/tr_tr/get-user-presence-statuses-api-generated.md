## Parametreler

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Evet |  |
| urlIdWS | string | query | Evet |  |
| userIds | string | query | Evet |  |

## Yanıt

Döndürür: [`GetUserPresenceStatusesResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/get_user_presence_statuses_response.py)

## Örnek

[inline-code-attrs-start title = 'get_user_presence_statuses Örneği'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.get_user_presence_statuses_response import GetUserPresenceStatusesResponse
from client.rest import ApiException
from pprint import pprint

# Hostu tanımlamak isteğe bağlıdır ve varsayılan https://fastcomments.com'tur
# Tüm desteklenen yapılandırma parametrelerinin listesi için configuration.py dosyasına bakın.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# API istemcisinin bir örneğiyle bir bağlam açın
with client.ApiClient(configuration) as api_client:
    # API sınıfının bir örneğini oluşturun
    api_instance = client.PublicApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    url_id_ws = 'url_id_ws_example' # str | 
    user_ids = 'user_ids_example' # str | 

    try:
        api_response = api_instance.get_user_presence_statuses(tenant_id, url_id_ws, user_ids)
        print("The response of PublicApi->get_user_presence_statuses:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling PublicApi->get_user_presence_statuses: %s\n" % e)
[inline-code-end]