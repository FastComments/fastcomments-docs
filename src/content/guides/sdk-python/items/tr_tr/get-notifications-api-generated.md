## Parametreler

| Ad | Tür | Konum | Zorunlu | Açıklama |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes |  |
| userId | string | query | No |  |
| urlId | string | query | No |  |
| fromCommentId | string | query | No |  |
| viewed | boolean | query | No |  |
| type | string | query | No |  |
| skip | number | query | No |  |

## Yanıt

Döndürür: [`GetNotificationsResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/get_notifications_response.py)

## Örnek

[inline-code-attrs-start title = 'get_notifications Örneği'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.get_notifications_response import GetNotificationsResponse
from client.rest import ApiException
from pprint import pprint

# Host tanımlamak isteğe bağlıdır ve varsayılan https://fastcomments.com'dur
# Tüm desteklenen yapılandırma parametrelerinin listesini görmek için configuration.py dosyasına bakın.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# İstemci, kimlik doğrulama ve yetkilendirme parametrelerini
# API sunucusunun güvenlik politikasına uygun şekilde yapılandırmalıdır.
# Aşağıda her bir kimlik doğrulama yöntemi için örnekler verilmiştir, kullanım durumunuza
# uygun olan örneği kullanın.

# API anahtarı ile yetkilendirmeyi yapılandırın: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# Gerekirse API anahtarı için önek (ör. Bearer) ayarlamak üzere aşağıdaki yorumu kaldırın
# configuration.api_key_prefix['api_key'] = 'Bearer'

# API istemcisi örneği ile bir bağlam içine girin
with client.ApiClient(configuration) as api_client:
    # API sınıfından bir örnek oluşturun
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    user_id = 'user_id_example' # str |  (isteğe bağlı)
    url_id = 'url_id_example' # str |  (isteğe bağlı)
    from_comment_id = 'from_comment_id_example' # str |  (isteğe bağlı)
    viewed = True # bool |  (isteğe bağlı)
    type = 'type_example' # str |  (isteğe bağlı)
    skip = 3.4 # float |  (isteğe bağlı)

    try:
        api_response = api_instance.get_notifications(tenant_id, user_id=user_id, url_id=url_id, from_comment_id=from_comment_id, viewed=viewed, type=type, skip=skip)
        print("The response of DefaultApi->get_notifications:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->get_notifications: %s\n" % e)
[inline-code-end]