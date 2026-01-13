## Parametreler

| Ad | Tür | Konum | Gerekli | Açıklama |
|------|------|----------|----------|-------------|
| tenantId | string | query | Hayır |  |

## Yanıt

Döndürür: [`AddHashTagsBulk200Response`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/add_hash_tags_bulk200_response.py)

## Örnek

[inline-code-attrs-start title = 'add_hash_tags_bulk Örnek'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.add_hash_tags_bulk200_response import AddHashTagsBulk200Response
from client.models.bulk_create_hash_tags_body import BulkCreateHashTagsBody
from client.rest import ApiException
from pprint import pprint

# Hostu tanımlamak isteğe bağlıdır ve varsayılan olarak https://fastcomments.com kullanılır
# Tüm desteklenen yapılandırma parametrelerinin listesi için configuration.py dosyasına bakın.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# İstemci, kimlik doğrulama ve yetkilendirme parametrelerini
# API sunucusunun güvenlik politikasına uygun şekilde yapılandırmalıdır.
# Aşağıda her kimlik doğrulama yöntemi için örnekler verilmiştir; 
# kimlik doğrulama kullanım durumunuza uyan örneği kullanın.

# API anahtarı yetkilendirmesini yapılandırın: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# Gerekirse API anahtarı için önek (ör. Bearer) ayarlamak üzere aşağıdaki satırı yorum dışı bırakın
# configuration.api_key_prefix['api_key'] = 'Bearer'

# API istemcisi örneği ile bir bağlama girin
with client.ApiClient(configuration) as api_client:
    # API sınıfından bir örnek oluşturun
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str |  (isteğe bağlı)
    bulk_create_hash_tags_body = client.BulkCreateHashTagsBody() # BulkCreateHashTagsBody |  (isteğe bağlı)

    try:
        api_response = api_instance.add_hash_tags_bulk(tenant_id=tenant_id, bulk_create_hash_tags_body=bulk_create_hash_tags_body)
        print("The response of DefaultApi->add_hash_tags_bulk:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->add_hash_tags_bulk: %s\n" % e)
[inline-code-end]