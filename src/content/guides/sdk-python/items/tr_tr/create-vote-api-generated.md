## Parametreler

| Ad | Tür | Konum | Gerekli | Açıklama |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes |  |
| commentId | string | query | Yes |  |
| direction | string | query | Yes |  |
| userId | string | query | No |  |
| anonUserId | string | query | No |  |

## Yanıt

Döndürür: [`VoteResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/vote_response.py)

## Örnek

[inline-code-attrs-start title = 'create_vote Örneği'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.api.default_api import CreateVoteOptions
from client.models.vote_response import VoteResponse
from client.rest import ApiException
from pprint import pprint

# Host tanımlama isteğe bağlıdır ve varsayılan olarak https://fastcomments.com adresine yönlendirilir
# Tüm desteklenen yapılandırma parametrelerinin listesi için configuration.py dosyasına bakın.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# İstemci, kimlik doğrulama ve yetkilendirme parametrelerini yapılandırmalıdır
# API sunucu güvenlik politikası doğrultusunda.
# Aşağıda her kimlik doğrulama yöntemi için örnekler sağlanmıştır, aşağıdaki örneklerden
# ihtiyacınıza uyanı kullanın.

# API anahtarı yetkilendirmesini yapılandır: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# Gerekirse API anahtarı için önek (örn. Bearer) ayarlamak için aşağıdaki satırı yorum dışı bırakın
# configuration.api_key_prefix['api_key'] = 'Bearer'

# API istemcisinin bir örneğiyle bir bağlam girin
with client.ApiClient(configuration) as api_client:
    # API sınıfının bir örneğini oluştur
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    comment_id = 'comment_id_example' # str | 
    direction = 'direction_example' # str | 
    user_id = 'user_id_example' # str |  (optional)
    anon_user_id = 'anon_user_id_example' # str |  (optional)

    try:
        api_response = api_instance.create_vote(tenant_id, comment_id, direction, CreateVoteOptions(user_id=user_id, anon_user_id=anon_user_id))
        print("DefaultApi->create_vote yanıtı:\n")
        pprint(api_response)
    except Exception as e:
        print("DefaultApi->create_vote çağrılırken oluşan istisna: %s\n" % e)
[inline-code-end]