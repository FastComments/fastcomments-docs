## Parametreler

| Ad | Tür | Konum | Gerekli | Açıklama |
|------|------|----------|----------|-------------|
| tenantId | string | query | Evet |  |
| tag | string | path | Evet |  |

## Yanıt

Döndürür: [`UpdateHashTagResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/update_hash_tag_response.py)

## Örnek

[inline-code-attrs-start title = 'patch_hash_tag Örneği'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.update_hash_tag_body import UpdateHashTagBody
from client.models.update_hash_tag_response import UpdateHashTagResponse
from client.rest import ApiException
from pprint import pprint

# Ana bilgisayarı tanımlamak isteğe bağlıdır ve varsayılan olarak https://fastcomments.com adresine ayarlanır
# configuration.py dosyasında desteklenen tüm yapılandırma parametrelerinin bir listesi bulunur.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# İstemcinin kimlik doğrulama ve yetkilendirme parametrelerini API sunucusunun güvenlik politikasına göre yapılandırması gerekir.
# Aşağıda her kimlik doğrulama yöntemi için örnekler verilmiştir; kimlik doğrulama kullanım durumunuza uyan örneği kullanın.

# API anahtarı yetkilendirmesini yapılandır: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# Gerekirse, API anahtarı için önek (ör. Bearer) ayarlamak için aşağıdaki satırı yorum dışı bırakın
# configuration.api_key_prefix['api_key'] = 'Bearer'

# API istemcisi örneği ile bir bağlam girin
with client.ApiClient(configuration) as api_client:
    # API sınıfının bir örneğini oluştur
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    tag = 'tag_example' # str | 
    update_hash_tag_body = client.UpdateHashTagBody() # UpdateHashTagBody |  (isteğe bağlı)

    try:
        api_response = api_instance.patch_hash_tag(tenant_id, tag, update_hash_tag_body)
        print("The response of DefaultApi->patch_hash_tag:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->patch_hash_tag: %s\n" % e)
[inline-code-end]