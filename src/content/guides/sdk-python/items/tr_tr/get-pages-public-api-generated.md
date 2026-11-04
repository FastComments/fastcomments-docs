Bir kiracı (tenant) için sayfaları listeler. FChat masaüstü istemcisi tarafından oda listesini doldurmak için kullanılır.
Her sayfa için çözümlenen özel yapılandırmada `enableFChat` değerinin true olması gerekir.
SSO gerektiren sayfalar, isteği yapan kullanıcının grup erişimine göre filtrelenir.

## Parametreler

| Ad | Tür | Konum | Zorunlu | Açıklama |
|------|------|----------|----------|-------------|
| tenantId | string | path | Evet |  |
| cursor | string | query | Hayır | Önceki bir istekten `nextCursor` olarak döndürülen opak sayfalama imleci. Aynı `sortBy` değerine bağlıdır. |
| limit | integer | query | Hayır | 1..200, varsayılan 50 |
| q | string | query | Hayır | İsteğe bağlı, büyük/küçük harfe duyarsız başlık öneki filtresi. |
| sortBy | string | query | Hayır | Sıralama düzeni. `updatedAt` (varsayılan, en yeni önce), `commentCount` (en çok yorumlu önce) veya `title` (alfabetik). |
| hasComments | boolean | query | Hayır | True ise, yalnızca en az bir yorumu olan sayfaları döndürür. |

## Yanıt

Döndürür: [`GetPublicPagesResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/get_public_pages_response.py)

## Örnek

[inline-code-attrs-start title = 'get_pages_public Örneği'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.api.public_api import GetPagesPublicOptions
from client.models.get_public_pages_response import GetPublicPagesResponse
from client.models.pages_sort_by import PagesSortBy
from client.rest import ApiException
from pprint import pprint

# Defining the host is optional and defaults to https://fastcomments.com
# See configuration.py for a list of all supported configuration parameters.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# Enter a context with an instance of the API client
with client.ApiClient(configuration) as api_client:
    # Create an instance of the API class
    api_instance = client.PublicApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    cursor = 'cursor_example' # str | Önceki bir istekten `nextCursor` olarak döndürülen opak sayfalama imleci. Aynı `sortBy` değerine bağlıdır. (optional)
    limit = 56 # int | 1..200, varsayılan 50 (optional)
    q = 'q_example' # str | İsteğe bağlı, büyük/küçük harfe duyarsız başlık öneki filtresi. (optional)
    sort_by = client.PagesSortBy() # PagesSortBy | Sıralama düzeni. `updatedAt` (varsayılan, en yeni önce), `commentCount` (en çok yorumlu önce) veya `title` (alfabetik). (optional)
    has_comments = True # bool | True ise, yalnızca en az bir yorumu olan sayfaları döndürür. (optional)

    try:
        api_response = api_instance.get_pages_public(tenant_id, GetPagesPublicOptions(cursor=cursor, limit=limit, q=q, sort_by=sort_by, has_comments=has_comments))
        print("The response of PublicApi->get_pages_public:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling PublicApi->get_pages_public: %s\n" % e)
[inline-code-end]