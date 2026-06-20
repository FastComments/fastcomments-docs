## Parametreler

| Ad | Tür | Konum | Gerekli | Açıklama |
|------|------|----------|----------|-------------|
| tenantId | string | path | Evet |  |
| urlId | string | query | Evet |  |

## Yanıt

Döndürür: [`GetV2PageReacts`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/get_v2_page_reacts.py)

## Örnek

[inline-code-attrs-start title = 'get_v2_page_reacts Örnek'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.get_v2_page_reacts import GetV2PageReacts
from client.rest import ApiException
from pprint import pprint

# Ana makineyi tanımlamak isteğe bağlıdır ve varsayılan https://fastcomments.com'dur
# Tüm desteklenen yapılandırma parametrelerinin bir listesini görmek için configuration.py dosyasına bakın.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# API istemcisi örneğiyle bir bağlama girin
with client.ApiClient(configuration) as api_client:
    # API sınıfından bir örnek oluşturun
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