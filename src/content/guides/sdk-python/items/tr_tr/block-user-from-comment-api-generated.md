## Parametreler

| Ad | Tür | Konum | Gerekli | Açıklama |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes |  |
| id | string | path | Yes |  |
| userId | string | query | No |  |
| anonUserId | string | query | No |  |

## Yanıt

Döndürür: [`BlockSuccess`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/block_success.py)

## Örnek

[inline-code-attrs-start title = 'block_user_from_comment Örneği'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.api.default_api import BlockUserFromCommentOptions
from client.models.block_from_comment_params import BlockFromCommentParams
from client.models.block_success import BlockSuccess
from client.rest import ApiException
from pprint import pprint

# Ana bilgisayarın tanımlanması isteğe bağlıdır ve varsayılan olarak https://fastcomments.com adresine yönlendirilir
# Tüm desteklenen yapılandırma parametrelerinin bir listesi için configuration.py dosyasına bakın.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# İstemcinin kimlik doğrulama ve yetkilendirme parametrelerini yapılandırması gerekir
# API sunucusu güvenlik politikasına uygun olarak.
# Her kimlik doğrulama yöntemi için örnekler aşağıda sağlanmıştır, şu örneği kullanın ki
# kimlik doğrulama kullanım durumunuzu karşılayan.

# API anahtarı yetkilendirmesini yapılandır: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# Gerekirse API anahtarı için önek (ör. Bearer) ayarlamak üzere aşağıdaki satırın yorumunu kaldırın
# configuration.api_key_prefix['api_key'] = 'Bearer'

# API istemcisi bir örnek ile bir bağlam girin
with client.ApiClient(configuration) as api_client:
    # API sınıfının bir örneğini oluştur
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    id = 'id_example' # str | 
    block_from_comment_params = client.BlockFromCommentParams() # BlockFromCommentParams | 
    user_id = 'user_id_example' # str |  (opsiyonel)
    anon_user_id = 'anon_user_id_example' # str |  (opsiyonel)

    try:
        api_response = api_instance.block_user_from_comment(tenant_id, id, block_from_comment_params, BlockUserFromCommentOptions(user_id=user_id, anon_user_id=anon_user_id))
        print("The response of DefaultApi->block_user_from_comment:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->block_user_from_comment: %s\n" % e)
[inline-code-end]