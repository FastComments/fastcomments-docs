## Parametreler

| Ad | Tür | Konum | Gerekli | Açıklama |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes |  |
| text-search | string | query | No |  |
| byIPFromComment | string | query | No |  |
| filters | string | query | No |  |
| searchFilters | string | query | No |  |
| afterId | string | query | No |  |
| demo | boolean | query | No |  |
| sso | string | query | No |  |

## Yanıt

Returns: [`ModerationAPIGetCommentIdsResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/moderation_api_get_comment_ids_response.py)

## Örnek

[inline-code-attrs-start title = 'get_api_ids Örneği'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.api.moderation_api import GetApiIdsOptions
from client.models.moderation_api_get_comment_ids_response import ModerationAPIGetCommentIdsResponse
from client.rest import ApiException
from pprint import pprint

# Host tanımlamak isteğe bağlıdır ve varsayılan olarak https://fastcomments.com adresine yönelir
# Tüm desteklenen yapılandırma parametrelerinin bir listesini configuration.py dosyasında bulabilirsiniz.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# API istemcisinin bir örneğiyle bir bağlam açın
with client.ApiClient(configuration) as api_client:
    # API sınıfının bir örneğini oluşturun
    api_instance = client.ModerationApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    text_search = 'text_search_example' # str |  (opsiyonel)
    by_ip_from_comment = 'by_ip_from_comment_example' # str |  (opsiyonel)
    filters = 'filters_example' # str |  (opsiyonel)
    search_filters = 'search_filters_example' # str |  (opsiyonel)
    after_id = 'after_id_example' # str |  (opsiyonel)
    demo = True # bool |  (opsiyonel)
    sso = 'sso_example' # str |  (opsiyonel)

    try:
        api_response = api_instance.get_api_ids(tenant_id, GetApiIdsOptions(text_search=text_search, by_ip_from_comment=by_ip_from_comment, filters=filters, search_filters=search_filters, after_id=after_id, demo=demo, sso=sso))
        print("ModerationApi->get_api_ids yanıtı:\n")
        pprint(api_response)
    except Exception as e:
        print("ModerationApi->get_api_ids çağrılırken istisna: %s\n" % e)
[inline-code-end]