List pages for a tenant. Used by the FChat desktop client to populate its room list.  
Requires `enableFChat` to be true on the resolved custom config for each page.  
Pages that require SSO are filtered against the requesting user's group access.

## Parameters

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Yes |  |
| cursor | string | query | No | Önceki bir istekte `nextCursor` olarak döndürülen opak sayfalama işaretçisi. Aynı `sortBy` ile ilişkilidir. |
| limit | integer | query | No | 1..200, varsayılan 50 |
| q | string | query | No | İsteğe bağlı büyük/küçük harfe duyarsız başlık önek filtresi. |
| sortBy | string | query | No | Sıralama düzeni. `updatedAt` (varsayılan, en yeni önce), `commentCount` (en çok yorum önce), veya `title` (alfabetik). |
| hasComments | boolean | query | No | True ise, yalnızca en az bir yorumu olan sayfaları döndür. |

## Response

Döndürür: [`GetPublicPagesResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/get_public_pages_response.py)

## Example

[inline-code-attrs-start title = 'get_pages_public Örnek'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.api.public_api import GetPagesPublicOptions
from client.models.get_public_pages_response import GetPublicPagesResponse
from client.models.pages_sort_by import PagesSortBy
from client.rest import ApiException
from pprint import pprint

# Host tanımlaması isteğe bağlıdır ve varsayılan olarak https://fastcomments.com adresine sahiptir
# Desteklenen tüm yapılandırma parametrelerinin listesini görmek için configuration.py dosyasına bakın.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# Enter a context with an instance of the API client
with client.ApiClient(configuration) as api_client:
    # API sınıfının bir örneğini oluştur
    api_instance = client.PublicApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    cursor = 'cursor_example' # str | Önceki bir istekte `nextCursor` olarak döndürülen opak sayfalama işaretçisi. Aynı `sortBy` ile ilişkilidir. (opsiyonel)
    limit = 56 # int | 1..200, varsayılan 50 (opsiyonel)
    q = 'q_example' # str | İsteğe bağlı büyük/küçük harfe duyarsız başlık önek filtresi. (opsiyonel)
    sort_by = client.PagesSortBy() # PagesSortBy | Sıralama düzeni. `updatedAt` (varsayılan, en yeni önce), `commentCount` (en çok yorum önce), veya `title` (alfabetik). (opsiyonel)
    has_comments = True # bool | True ise, yalnızca en az bir yorumu olan sayfaları döndür. (opsiyonel)

    try:
        api_response = api_instance.get_pages_public(tenant_id, GetPagesPublicOptions(cursor=cursor, limit=limit, q=q, sort_by=sort_by, has_comments=has_comments))
        print("The response of PublicApi->get_pages_public:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling PublicApi->get_pages_public: %s\n" % e)
[inline-code-end]