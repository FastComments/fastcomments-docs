## Parametreler

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Evet |  |
| id | string | path | Evet |  |

## Yanıt

Döndürür: [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/api_empty_response.py)

## Örnek

[inline-code-attrs-start title = 'delete_question_config Örneği'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.api_empty_response import APIEmptyResponse
from client.rest import ApiException
from pprint import pprint

# Sunucuyu tanımlamak isteğe bağlıdır ve varsayılan olarak https://fastcomments.com kullanılır
# Tüm desteklenen yapılandırma parametrelerinin listesi için configuration.py dosyasına bakın.
# İstemci kimlik doğrulama ve yetkilendirme parametrelerini
# API sunucusunun güvenlik politikasına uygun olarak yapılandırmalıdır.
# Her kimlik doğrulama yöntemi için örnekler aşağıda verilmiştir, ihtiyacınıza uygun olan
# örneği kullanın.
# Configure API key authorization: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# Gerekirse API anahtarı için önek (ör. Bearer) ayarlamak üzere aşağıdaki satırı yorumdan çıkarın
# configuration.api_key_prefix['api_key'] = 'Bearer'

# API istemcisinin bir örneğiyle bir bağlam içine girin
with client.ApiClient(configuration) as api_client:
    # API sınıfının bir örneğini oluşturun
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    id = 'id_example' # str | 

    try:
        api_response = api_instance.delete_question_config(tenant_id, id)
        print("The response of DefaultApi->delete_question_config:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->delete_question_config: %s\n" % e)
[inline-code-end]