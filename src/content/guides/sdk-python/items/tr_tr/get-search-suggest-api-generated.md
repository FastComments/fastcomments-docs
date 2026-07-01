## Parameters

| Ad | Tür | Konum | Gerekli | Açıklama |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes |  |
| text-search | string | query | No |  |
| sso | string | query | No |  |

## Yanıt

Döndürür: [`ModerationSuggestResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/moderation_suggest_response.py)

## Örnek

[inline-code-attrs-start title = 'get_search_suggest Örneği'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.api.moderation_api import GetSearchSuggestOptions
from client.models.moderation_suggest_response import ModerationSuggestResponse
from client.rest import ApiException
from pprint import pprint

# Host tanımlaması isteğe bağlıdır ve varsayılan olarak https://fastcomments.com kullanılır
# configuration.py dosyasında desteklenen tüm yapılandırma parametrelerinin bir listesi bulunur.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# API istemcisi örneğiyle bir bağlam açın
with client.ApiClient(configuration) as api_client:
    # API sınıfının bir örneğini oluşturun
    api_instance = client.ModerationApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    text_search = 'text_search_example' # str |  (optional)
    sso = 'sso_example' # str |  (optional)

    try:
        api_response = api_instance.get_search_suggest(tenant_id, GetSearchSuggestOptions(text_search=text_search, sso=sso))
        print("The response of ModerationApi->get_search_suggest:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling ModerationApi->get_search_suggest: %s\n" % e)
[inline-code-end]