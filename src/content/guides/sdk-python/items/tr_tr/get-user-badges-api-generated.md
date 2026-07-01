## Parametreler

| İsim | Tip | Konum | Gereklidir | Açıklama |
|------|------|----------|------------|-------------|
| tenantId | string | query | Evet |  |
| userId | string | query | Hayır |  |
| badgeId | string | query | Hayır |  |
| type | number | query | Hayır |  |
| displayedOnComments | boolean | query | Hayır |  |
| limit | number | query | Hayır |  |
| skip | number | query | Hayır |  |

## Yanıt

Döndürür: [`APIGetUserBadgesResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/api_get_user_badges_response.py)

## Örnek

[inline-code-attrs-start title = 'get_user_badges Örneği'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.api.default_api import GetUserBadgesOptions
from client.models.api_get_user_badges_response import APIGetUserBadgesResponse
from client.rest import ApiException
from pprint import pprint

# Host tanımlama isteğe bağlıdır ve varsayılan olarak https://fastcomments.com adresini kullanır
# Tüm desteklenen yapılandırma parametrelerinin listesi için configuration.py dosyasına bakın.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# İstemci, API sunucusunun güvenlik politikası doğrultusunda kimlik doğrulama ve yetkilendirme parametrelerini yapılandırmalıdır.
# Aşağıda her kimlik doğrulama yöntemi için örnekler verilmiştir; ihtiyaçlarınıza uygun örneği kullanın.

# API anahtarı yetkilendirmesini yapılandır: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# Gerekirse API anahtarı için önek (örn. Bearer) ayarlamak için aşağıdaki satırı yorumdan çıkarın
# configuration.api_key_prefix['api_key'] = 'Bearer'

# API istemcisi örneğiyle bir bağlam girin
with client.ApiClient(configuration) as api_client:
    # API sınıfının bir örneğini oluştur
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    user_id = 'user_id_example' # str |  (optional)
    badge_id = 'badge_id_example' # str |  (optional)
    type = 3.4 # float |  (optional)
    displayed_on_comments = True # bool |  (optional)
    limit = 3.4 # float |  (optional)
    skip = 3.4 # float |  (optional)

    try:
        api_response = api_instance.get_user_badges(tenant_id, GetUserBadgesOptions(user_id=user_id, badge_id=badge_id, type=type, displayed_on_comments=displayed_on_comments, limit=limit, skip=skip))
        print("The response of DefaultApi->get_user_badges:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->get_user_badges: %s\n" % e)
[inline-code-end]