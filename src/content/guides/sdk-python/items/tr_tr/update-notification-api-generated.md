## Parametreler

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Evet |  |
| id | string | path | Evet |  |
| userId | string | query | Hayır |  |

## Yanıt

Döndürür: [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/api_empty_response.py)

## Örnek

[inline-code-attrs-start title = 'update_notification Örneği'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.api_empty_response import APIEmptyResponse
from client.models.update_notification_body import UpdateNotificationBody
from client.rest import ApiException
from pprint import pprint

# Sunucunun tanımlanması isteğe bağlıdır ve varsayılan https://fastcomments.com'dur
# configuration.py dosyasına bakarak desteklenen tüm yapılandırma parametrelerinin listesine ulaşabilirsiniz.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# İstemci, kimlik doğrulama ve yetkilendirme parametrelerini
# API sunucusunun güvenlik politikasına uygun olarak yapılandırmalıdır.
# Her bir kimlik doğrulama yöntemi için örnekler aşağıda sağlanmıştır; kullanım durumunuza uygun olanı kullanın.

# API anahtarı yetkilendirmesini yapılandırın: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# Gerekirse API anahtarı için öneki (ör. Bearer) ayarlamak için aşağıyı yorumdan çıkarın
# configuration.api_key_prefix['api_key'] = 'Bearer'

# API istemcisi örneği ile bir bağlama girin
with client.ApiClient(configuration) as api_client:
    # API sınıfından bir örnek oluşturun
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    id = 'id_example' # str | 
    update_notification_body = client.UpdateNotificationBody() # UpdateNotificationBody | 
    user_id = 'user_id_example' # str |  (isteğe bağlı)

    try:
        api_response = api_instance.update_notification(tenant_id, id, update_notification_body, user_id=user_id)
        print("The response of DefaultApi->update_notification:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->update_notification: %s\n" % e)
[inline-code-end]