## Parametreler

| İsim | Tür | Konum | Gerekli | Açıklama |
|------|------|----------|----------|-------------|
| tenantId | string | query | Evet |  |

## Yanıt

Döndürür: [`CreateQuestionConfig200Response`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/create_question_config200_response.py)

## Örnek

[inline-code-attrs-start title = 'create_question_config Örneği'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.create_question_config200_response import CreateQuestionConfig200Response
from client.models.create_question_config_body import CreateQuestionConfigBody
from client.rest import ApiException
from pprint import pprint

# Sunucuyu (host) tanımlamak isteğe bağlıdır ve varsayılan olarak https://fastcomments.com kullanılır
# Tüm desteklenen yapılandırma parametrelerinin listesi için configuration.py'e bakın.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# İstemci, kimlik doğrulama ve yetkilendirme parametrelerini
# API sunucusunun güvenlik politikasıyla uyumlu şekilde yapılandırmalıdır.
# Her bir kimlik doğrulama yöntemi için örnekler aşağıda verilmiştir, kullanım durumunuza uygun örneği kullanın.

# API anahtarı yetkilendirmesini yapılandırın: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# Gerekirse API anahtarı için öneki (ör. Bearer) ayarlamak için aşağıdaki satırı yorumdan çıkarın
# configuration.api_key_prefix['api_key'] = 'Bearer'

# API istemcisi örneği ile bir bağlam içine girin
with client.ApiClient(configuration) as api_client:
    # API sınıfından bir örnek oluşturun
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    create_question_config_body = client.CreateQuestionConfigBody() # CreateQuestionConfigBody | 

    try:
        api_response = api_instance.create_question_config(tenant_id, create_question_config_body)
        print("The response of DefaultApi->create_question_config:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->create_question_config: %s\n" % e)
[inline-code-end]