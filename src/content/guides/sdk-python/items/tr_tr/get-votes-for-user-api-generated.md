## Parametreler

| Ad | Tür | Konum | Gerekli | Açıklama |
|------|------|----------|----------|-------------|
| tenantId | string | query | Evet |  |
| urlId | string | query | Evet |  |
| userId | string | query | Hayır |  |
| anonUserId | string | query | Hayır |  |

## Yanıt

Döndürür: [`GetVotesForUserResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/get_votes_for_user_response.py)

## Örnek

[inline-code-attrs-start title = 'get_votes_for_user Örneği'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.get_votes_for_user_response import GetVotesForUserResponse
from client.rest import ApiException
from pprint import pprint

# Host tanımlamak isteğe bağlıdır ve varsayılan olarak https://fastcomments.com kullanılır
# Tüm desteklenen yapılandırma parametrelerinin listesi için configuration.py dosyasına bakın.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# İstemci kimlik doğrulama ve yetkilendirme parametrelerini yapılandırmalıdır
# API sunucusunun güvenlik politikasına uygun şekilde.
# Her kimlik doğrulama yöntemi için örnekler aşağıda verilmiştir;
# kullanım durumunuza uyan örneği kullanın.

# Configure API key authorization: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# Gerekirse API anahtarı için önek (ör. Bearer) ayarlamak üzere aşağıdaki satırı yorum dışı bırakın
# configuration.api_key_prefix['api_key'] = 'Bearer'

# API istemcisinin bir örneğiyle bir bağlama girin
with client.ApiClient(configuration) as api_client:
    # API sınıfının bir örneğini oluşturun
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    url_id = 'url_id_example' # str | 
    user_id = 'user_id_example' # str |  (isteğe bağlı)
    anon_user_id = 'anon_user_id_example' # str |  (isteğe bağlı)

    try:
        api_response = api_instance.get_votes_for_user(tenant_id, url_id, user_id=user_id, anon_user_id=anon_user_id)
        print("The response of DefaultApi->get_votes_for_user:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->get_votes_for_user: %s\n" % e)
[inline-code-end]