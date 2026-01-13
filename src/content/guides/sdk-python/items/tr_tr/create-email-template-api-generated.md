## Parametreler

| Ad | Tür | Konum | Gerekli | Açıklama |
|------|------|----------|----------|-------------|
| tenantId | string | query | Evet |  |

## Yanıt

Döndürür: [`CreateEmailTemplate200Response`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/create_email_template200_response.py)

## Örnek

[inline-code-attrs-start title = 'create_email_template Örneği'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.create_email_template200_response import CreateEmailTemplate200Response
from client.models.create_email_template_body import CreateEmailTemplateBody
from client.rest import ApiException
from pprint import pprint

# Sunucunun tanımlanması isteğe bağlıdır ve varsayılan https://fastcomments.com'tur
# Tüm desteklenen yapılandırma parametrelerinin listesi için configuration.py dosyasına bakın.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# İstemci, kimlik doğrulama ve yetkilendirme parametrelerini yapılandırmalıdır
# API sunucusunun güvenlik politikasıyla uyumlu olarak.
# Her kimlik doğrulama yöntemi için örnekler aşağıda verilmiştir, ihtiyaç duyduğunuz
# kimlik doğrulama durumunu karşılayan örneği kullanın.

# Configure API key authorization: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# Gerekirse API anahtarı için ön eki (örn. Bearer) ayarlamak üzere aşağıdakinin yorumunu kaldırın
# configuration.api_key_prefix['api_key'] = 'Bearer'

# API istemci örneği ile bir bağlam açın
with client.ApiClient(configuration) as api_client:
    # API sınıfından bir örnek oluşturun
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    create_email_template_body = client.CreateEmailTemplateBody() # CreateEmailTemplateBody | 

    try:
        api_response = api_instance.create_email_template(tenant_id, create_email_template_body)
        print("The response of DefaultApi->create_email_template:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->create_email_template: %s\n" % e)
[inline-code-end]

---