## Parametreler

| İsim | Tür | Konum | Gerekli | Açıklama |
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
from client.api.default_api import GetNotificationsOptions
from client.models.get_notifications_response import GetNotificationsResponse
from client.rest import ApiException
from pprint import pprint

# Host tanımlama isteğe bağlıdır ve varsayılan olarak https://fastcomments.com adresine yönelir
# Tüm desteklenen yapılandırma parametrelerinin listesi için configuration.py dosyasına bakın.
# İstemci, API sunucusunun güvenlik politikasına uygun olarak kimlik doğrulama ve yetkilendirme parametrelerini yapılandırmalıdır.
# Her kimlik doğrulama yöntemi için örnekler aşağıda sağlanmıştır; kullanım durumunuza uyan örneği kullanın.

# API anahtarı yetkilendirmesini yapılandır: api_key
# Gerekirse API anahtarı için önek (ör. Bearer) ayarlamak için aşağıdaki satırın yorumunu kaldırın
# configuration.api_key_prefix['api_key'] = 'Bearer'

# API istemcisinin bir örneğiyle bir bağlam girin
with client.ApiClient(configuration) as api_client:
    # API sınıfının bir örneğini oluştur
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    user_id = 'user_id_example' # str |  (isteğe bağlı)
    url_id = 'url_id_example' # str |  (isteğe bağlı)
    from_comment_id = 'from_comment_id_example' # str |  (isteğe bağlı)
    viewed = True # bool |  (isteğe bağlı)
    type = 'type_example' # str |  (isteğe bağlı)
    skip = 3.4 # float |  (isteğe bağlı)

    try:
        api_response = api_instance.get_notifications(tenant_id, GetNotificationsOptions(user_id=user_id, url_id=url_id, from_comment_id=from_comment_id, viewed=viewed, type=type, skip=skip))
        print("DefaultApi->get_notifications yanıtı:\n")
        pprint(api_response)
    except Exception as e:
        print("DefaultApi->get_notifications çağrılırken istisna: %s\n" % e)
[inline-code-end]