Bir kiracı için sayfaları listeler. FChat masaüstü istemcisi tarafından oda listesini doldurmak için kullanılır.
Her sayfa için çözümlenmiş özel yapılandırmada `enableFChat`'in true olması gerekir.
SSO gerektiren sayfalar, istekte bulunan kullanıcının grup erişimine göre filtrelenir.

## Parametreler

| Ad | Tip | Konum | Gerekli | Açıklama |
|------|------|----------|----------|-------------|
| tenantId | string | path | Evet |  |
| cursor | string | query | Hayır | Önceki bir istekte `nextCursor` olarak döndürülen opak sayfalama göstergesi. Aynı `sortBy` ile ilişkilidir. |
| limit | integer | query | Hayır | 1..200, varsayılan 50 |
| q | string | query | Hayır | Opsiyonel, büyük/küçük harf duyarsız başlık önek filtresi. |
| sortBy | string | query | Hayır | Sıralama düzeni. `updatedAt` (varsayılan, en yeni önce), `commentCount` (en çok yorum olan önce), veya `title` (alfabetik). |
| hasComments | boolean | query | Hayır | Eğer true ise, yalnızca en az bir yorumu olan sayfaları döndürür. |

## Yanıt

Döndürür: [`GetPublicPagesResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/get_public_pages_response.py)

## Örnek

[inline-code-attrs-start title = 'get_pages_public Örnek'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.get_public_pages_response import GetPublicPagesResponse
from client.models.pages_sort_by import PagesSortBy
from client.rest import ApiException
from pprint import pprint

# Host'u tanımlamak isteğe bağlıdır ve varsayılan https://fastcomments.com'tur
# Tüm desteklenen yapılandırma parametrelerinin listesi için configuration.py dosyasına bakın.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# API istemcisinin bir örneğiyle bir bağlam başlatın
with client.ApiClient(configuration) as api_client:
    # API sınıfının bir örneğini oluşturun
    api_instance = client.PublicApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    cursor = 'cursor_example' # str | Önceki bir istekte `nextCursor` olarak döndürülen opak sayfalama göstergesi. Aynı `sortBy` ile ilişkilidir. (opsiyonel)
    limit = 56 # int | 1..200, varsayılan 50 (opsiyonel)
    q = 'q_example' # str | Opsiyonel, büyük/küçük harf duyarsız başlık önek filtresi. (opsiyonel)
    sort_by = client.PagesSortBy() # PagesSortBy | Sıralama düzeni. `updatedAt` (varsayılan, en yeni önce), `commentCount` (en çok yorum olan önce), veya `title` (alfabetik). (opsiyonel)
    has_comments = True # bool | Eğer true ise, yalnızca en az bir yorumu olan sayfaları döndürür. (opsiyonel)

    try:
        api_response = api_instance.get_pages_public(tenant_id, cursor=cursor, limit=limit, q=q, sort_by=sort_by, has_comments=has_comments)
        print("The response of PublicApi->get_pages_public:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling PublicApi->get_pages_public: %s\n" % e)
[inline-code-end]