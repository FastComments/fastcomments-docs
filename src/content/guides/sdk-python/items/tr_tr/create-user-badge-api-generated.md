## Parametreler

| Ad | Tür | Konum | Gerekli | Açıklama |
|------|------|----------|----------|-------------|
| tenantId | string | query | Evet |  |

## Yanıt

Döndürür: [`APICreateUserBadgeResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/api_create_user_badge_response.py)

## Örnek

[inline-code-attrs-start title = 'create_user_badge Örneği'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.api_create_user_badge_response import APICreateUserBadgeResponse
from client.models.create_user_badge_params import CreateUserBadgeParams
from client.rest import ApiException
from pprint import pprint

# Host tanımlamak isteğe bağlıdır ve varsayılan https://fastcomments.com'dur
# Tüm desteklenen yapılandırma parametreleri için configuration.py dosyasına bakın.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# İstemci, kimlik doğrulama ve yetkilendirme parametrelerini
# API sunucusunun güvenlik politikasına göre yapılandırmalıdır.
# Her kimlik doğrulama yöntemi için örnekler aşağıda verilmiştir, kullanım durumunuza uyan
# örneği kullanın.

# API anahtarı yetkilendirmesini yapılandırın: api_key
# Gerekirse API anahtarı için önek (ör. Bearer) ayarlamak için aşağıdaki satırın başındaki yorum işaretini kaldırın
# configuration.api_key_prefix['api_key'] = 'Bearer'

# API istemcisi örneğiyle bir bağlam açın
with client.ApiClient(configuration) as api_client:
    # API sınıfından bir örnek oluşturun
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    create_user_badge_params = client.CreateUserBadgeParams() # CreateUserBadgeParams | 

    try:
        api_response = api_instance.create_user_badge(tenant_id, create_user_badge_params)
        print("The response of DefaultApi->create_user_badge:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->create_user_badge: %s\n" % e)
[inline-code-end]