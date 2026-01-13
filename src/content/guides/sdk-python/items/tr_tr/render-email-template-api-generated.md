## Parametreler

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Evet |  |
| locale | string | query | Hayır |  |

## Yanıt

Döndürür: [`RenderEmailTemplate200Response`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/render_email_template200_response.py)

## Örnek

[inline-code-attrs-start title = 'render_email_template Örneği'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.render_email_template200_response import RenderEmailTemplate200Response
from client.models.render_email_template_body import RenderEmailTemplateBody
from client.rest import ApiException
from pprint import pprint

# Host tanımlamak isteğe bağlıdır ve varsayılan https://fastcomments.com'dur
# Tüm desteklenen yapılandırma parametrelerinin listesi için configuration.py'ye bakın.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# İstemci, kimlik doğrulama ve yetkilendirme parametrelerini
# API sunucusunun güvenlik politikasına göre yapılandırmalıdır.
# Her bir kimlik doğrulama yöntemi için örnekler aşağıda sağlanmıştır, kullanım
# durumunuza uyan örneği kullanın.

# Configure API key authorization: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# Gerekirse API anahtarı için önek (ör. Bearer) ayarlamak için aşağıdaki satırı yorumdan çıkarın
# configuration.api_key_prefix['api_key'] = 'Bearer'

# API istemcisinin bir örneği ile bir bağlam girin
with client.ApiClient(configuration) as api_client:
    # API sınıfından bir örnek oluşturun
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    render_email_template_body = client.RenderEmailTemplateBody() # RenderEmailTemplateBody | 
    locale = 'locale_example' # str |  (optional)

    try:
        api_response = api_instance.render_email_template(tenant_id, render_email_template_body, locale=locale)
        print("The response of DefaultApi->render_email_template:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->render_email_template: %s\n" % e)
[inline-code-end]