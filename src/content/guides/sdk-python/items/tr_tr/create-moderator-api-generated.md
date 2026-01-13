## Parametreler

| Ad | Tür | Konum | Gerekli | Açıklama |
|------|------|----------|----------|-------------|
| tenantId | string | query | Evet |  |

## Yanıt

Dönüş: [`CreateModerator200Response`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/create_moderator200_response.py)

## Örnek

[inline-code-attrs-start title = 'create_moderator Örneği'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.create_moderator200_response import CreateModerator200Response
from client.models.create_moderator_body import CreateModeratorBody
from client.rest import ApiException
from pprint import pprint

# Sunucuyu tanımlamak isteğe bağlıdır ve varsayılan https://fastcomments.com'tur
# Tüm desteklenen yapılandırma parametrelerinin listesi için configuration.py dosyasına bakın.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# İstemci, kimlik doğrulama ve yetkilendirme parametrelerini
# API sunucusunun güvenlik politikasına uygun şekilde yapılandırmalıdır.
# Aşağıda her kimlik doğrulama yöntemi için örnekler verilmiştir, kullanım durumunuza
# uyan örneği kullanın.

# API anahtarı yetkilendirmesini yapılandırın: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# Gerekirse API anahtarı için önek (örn. Bearer) ayarlamak üzere aşağıdakinin yorumunu kaldırın
# configuration.api_key_prefix['api_key'] = 'Bearer'

# Bir API istemcisi örneğiyle bağlama girin
with client.ApiClient(configuration) as api_client:
    # API sınıfından bir örnek oluşturun
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    create_moderator_body = client.CreateModeratorBody() # CreateModeratorBody | 

    try:
        api_response = api_instance.create_moderator(tenant_id, create_moderator_body)
        print("The response of DefaultApi->create_moderator:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->create_moderator: %s\n" % e)
[inline-code-end]