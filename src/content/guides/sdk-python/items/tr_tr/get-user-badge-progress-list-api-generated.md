## Parametreler

| Ad | Tür | Konum | Gerekli | Açıklama |
|------|------|----------|----------|-------------|
| tenantId | string | query | Evet |  |
| userId | string | query | Hayır |  |
| limit | number | query | Hayır |  |
| skip | number | query | Hayır |  |

## Yanıt

Döndürür: [`APIGetUserBadgeProgressListResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/api_get_user_badge_progress_list_response.py)

## Örnek

[inline-code-attrs-start title = 'get_user_badge_progress_list Örneği'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.api.default_api import GetUserBadgeProgressListOptions
from client.models.api_get_user_badge_progress_list_response import APIGetUserBadgeProgressListResponse
from client.rest import ApiException
from pprint import pprint

# Host tanımlama isteğe bağlıdır ve varsayılan olarak https://fastcomments.com adresine ayarlanır
# Tüm desteklenen yapılandırma parametrelerinin listesi için configuration.py dosyasına bakın.
# İstemcinin kimlik doğrulama ve yetkilendirme parametrelerini API sunucusunun güvenlik politikasına göre yapılandırması gerekir.
# API sunucusunun güvenlik politikasına uygun olarak.
# Her kimlik doğrulama yöntemi için örnekler aşağıda sağlanmıştır, örneği kullanın ki
# kimlik doğrulama kullanım durumunuzu karşılasın.
# API anahtarı yetkilendirmesini yapılandır: api_key
# Gerekirse API anahtarı için önek (örn. Bearer) ayarlamak için aşağıdaki satırın yorumunu kaldırın
# Uncomment below to setup prefix (e.g. Bearer) for API key, if needed
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# The client must configure the authentication and authorization parameters
# in accordance with the API server security policy.
# Examples for each auth method are provided below, use the example that
# satisfies your auth use case.

# Configure API key authorization: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# Uncomment below to setup prefix (e.g. Bearer) for API key, if needed
# configuration.api_key_prefix['api_key'] = 'Bearer'

# Enter a context with an instance of the API client
with client.ApiClient(configuration) as api_client:
    # Create an instance of the API class
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    user_id = 'user_id_example' # str |  (opsiyonel)
    limit = 3.4 # float |  (opsiyonel)
    skip = 3.4 # float |  (opsiyonel)

    try:
        api_response = api_instance.get_user_badge_progress_list(tenant_id, GetUserBadgeProgressListOptions(user_id=user_id, limit=limit, skip=skip))
        print("The response of DefaultApi->get_user_badge_progress_list:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->get_user_badge_progress_list: %s\n" % e)
[inline-code-end]