## Parametreler

| Ad | Tip | Konum | Gerekli | Açıklama |
|------|------|----------|----------|-------------|
| tenantId | string | query | Evet |  |
| commentId | string | query | Evet |  |
| direction | string | query | Evet |  |
| userId | string | query | Hayır |  |
| anonUserId | string | query | Hayır |  |

## Yanıt

Döndürür: [`VoteResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/vote_response.py)

## Örnek

[inline-code-attrs-start title = 'create_vote Örneği'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.vote_response import VoteResponse
from client.rest import ApiException
from pprint import pprint

# Host tanımlamak isteğe bağlıdır ve varsayılan olarak https://fastcomments.com kullanılır
# Tüm desteklenen yapılandırma parametreleri listesini görmek için configuration.py dosyasına bakın.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# İstemci, kimlik doğrulama ve yetkilendirme parametrelerini
# API sunucusunun güvenlik politikasına uygun şekilde yapılandırmalıdır.
# Aşağıda her kimlik doğrulama yöntemi için örnekler verilmiştir; kendi kullanım durumunuza uyan örneği kullanın.

# API anahtarı yetkilendirmesini yapılandırın: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# Gerekirse API anahtarı için öneki (ör. Bearer) ayarlamak için aşağıdaki satırı yorum dışı bırakın
# configuration.api_key_prefix['api_key'] = 'Bearer'

# API istemcisi örneği ile bir bağlam içine girin
with client.ApiClient(configuration) as api_client:
    # API sınıfından bir örnek oluşturun
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    comment_id = 'comment_id_example' # str | 
    direction = 'direction_example' # str | 
    user_id = 'user_id_example' # str |  (isteğe bağlı)
    anon_user_id = 'anon_user_id_example' # str |  (isteğe bağlı)

    try:
        api_response = api_instance.create_vote(tenant_id, comment_id, direction, user_id=user_id, anon_user_id=anon_user_id)
        print("The response of DefaultApi->create_vote:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->create_vote: %s\n" % e)
[inline-code-end]