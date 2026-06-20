## Parametreler

| Ad | Tür | Konum | Gerekli | Açıklama |
|------|------|----------|----------|-------------|
| tenantId | string | query | Evet |  |
| isLive | boolean | query | Hayır |  |
| doSpamCheck | boolean | query | Hayır |  |
| sendEmails | boolean | query | Hayır |  |
| populateNotifications | boolean | query | Hayır |  |

## Yanıt

Döndürür: [`APISaveCommentResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/api_save_comment_response.py)

## Örnek

[inline-code-attrs-start title = 'save_comment Örneği'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.api_save_comment_response import APISaveCommentResponse
from client.models.create_comment_params import CreateCommentParams
from client.rest import ApiException
from pprint import pprint

# Defining the host is optional and defaults to https://fastcomments.com
# Desteklenen tüm yapılandırma parametrelerinin listesi için configuration.py dosyasına bakın.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# The client must configure the authentication and authorization parameters
# in accordance with the API server security policy.
# İstemci, kimlik doğrulama ve yetkilendirme parametrelerini yapılandırmalıdır
# API sunucusunun güvenlik politikasına uygun olarak.
# Her kimlik doğrulama yöntemi için örnekler aşağıda verilmiştir;
# kullanım durumunuza uygun olanı kullanın.
# Configure API key authorization: api_key
# API anahtarı yetkilendirmesini yapılandırın: api_key
# Uncomment below to setup prefix (e.g. Bearer) for API key, if needed
# Gerekliyse API anahtarı için öneki (ör. Bearer) ayarlamak üzere aşağıdaki satırın yorumunu kaldırın
# configuration.api_key_prefix['api_key'] = 'Bearer'

# Enter a context with an instance of the API client
with client.ApiClient(configuration) as api_client:
    # Create an instance of the API class
    # API istemcisi örneği ile bir bağlam açın
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    create_comment_params = client.CreateCommentParams() # CreateCommentParams | 
    is_live = True # bool |  (isteğe bağlı)
    do_spam_check = True # bool |  (isteğe bağlı)
    send_emails = True # bool |  (isteğe bağlı)
    populate_notifications = True # bool |  (isteğe bağlı)

    try:
        api_response = api_instance.save_comment(tenant_id, create_comment_params, is_live=is_live, do_spam_check=do_spam_check, send_emails=send_emails, populate_notifications=populate_notifications)
        print("The response of DefaultApi->save_comment:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->save_comment: %s\n" % e)
[inline-code-end]

---