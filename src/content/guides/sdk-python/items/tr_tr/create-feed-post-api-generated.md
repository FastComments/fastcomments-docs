## Parametreler

| Ad | Tür | Konum | Gerekli | Açıklama |
|------|------|----------|----------|-------------|
| tenantId | string | query | Evet |  |
| broadcastId | string | query | Hayır |  |
| isLive | boolean | query | Hayır |  |
| doSpamCheck | boolean | query | Hayır |  |
| skipDupCheck | boolean | query | Hayır |  |

## Yanıt

Döndürür: [`CreateFeedPostsResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/create_feed_posts_response.py)

## Örnek

[inline-code-attrs-start title = 'create_feed_post Örneği'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.create_feed_post_params import CreateFeedPostParams
from client.models.create_feed_posts_response import CreateFeedPostsResponse
from client.rest import ApiException
from pprint import pprint

# Sunucuyu tanımlamak isteğe bağlıdır ve varsayılan https://fastcomments.com'tur
# Desteklenen tüm yapılandırma parametrelerinin listesi için configuration.py dosyasına bakın.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# İstemci, kimlik doğrulama ve yetkilendirme parametrelerini
# API sunucusunun güvenlik politikasına uygun şekilde yapılandırmalıdır.
# Her kimlik doğrulama yöntemi için örnekler aşağıda sağlanmıştır; 
# kimlik doğrulama kullanım durumunuza uyan örneği kullanın.

# Configure API key authorization: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# Gerekirse API anahtarı için önek (örn. Bearer) ayarlamak için aşağıdakinin yorumunu kaldırın
# configuration.api_key_prefix['api_key'] = 'Bearer'

# API istemcisi örneği ile bir bağlam içerisine girin
with client.ApiClient(configuration) as api_client:
    # API sınıfından bir örnek oluşturun
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    create_feed_post_params = client.CreateFeedPostParams() # CreateFeedPostParams | 
    broadcast_id = 'broadcast_id_example' # str |  (optional)
    is_live = True # bool |  (optional)
    do_spam_check = True # bool |  (optional)
    skip_dup_check = True # bool |  (optional)

    try:
        api_response = api_instance.create_feed_post(tenant_id, create_feed_post_params, broadcast_id=broadcast_id, is_live=is_live, do_spam_check=do_spam_check, skip_dup_check=skip_dup_check)
        print("The response of DefaultApi->create_feed_post:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->create_feed_post: %s\n" % e)
[inline-code-end]

---