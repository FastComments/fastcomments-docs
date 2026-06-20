## Parametreler

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Evet |  |
| isLive | boolean | query | Hayır |  |
| doSpamCheck | boolean | query | Hayır |  |
| sendEmails | boolean | query | Hayır |  |
| populateNotifications | boolean | query | Hayır |  |

## Yanıt

Döndürür: [`SaveCommentsBulkResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/save_comments_bulk_response.py)

## Örnek

[inline-code-attrs-start title = 'save_comments_bulk Örneği'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.create_comment_params import CreateCommentParams
from client.models.save_comments_bulk_response import SaveCommentsBulkResponse
from client.rest import ApiException
from pprint import pprint

# Host'u belirtmek isteğe bağlıdır ve varsayılan https://fastcomments.com'dur
# Tüm desteklenen yapılandırma parametrelerinin listesi için configuration.py dosyasına bakın.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# İstemci, kimlik doğrulama ve yetkilendirme parametrelerini yapılandırmalıdır
# API sunucusu güvenlik politikasına uygun olarak.
# Aşağıda her bir kimlik doğrulama yöntemi için örnekler verilmiştir, kullanım durumunuza uygun olanı kullanın.
# Configure API key authorization: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# Uncomment below to setup prefix (e.g. Bearer) for API key, if needed
# configuration.api_key_prefix['api_key'] = 'Bearer'

# API istemcisi örneği ile bir bağlam açın
with client.ApiClient(configuration) as api_client:
    # API sınıfından bir örnek oluşturun
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    create_comment_params = [client.CreateCommentParams()] # List[CreateCommentParams] | 
    is_live = True # bool |  (isteğe bağlı)
    do_spam_check = True # bool |  (isteğe bağlı)
    send_emails = True # bool |  (isteğe bağlı)
    populate_notifications = True # bool |  (isteğe bağlı)

    try:
        api_response = api_instance.save_comments_bulk(tenant_id, create_comment_params, is_live=is_live, do_spam_check=do_spam_check, send_emails=send_emails, populate_notifications=populate_notifications)
        print("The response of DefaultApi->save_comments_bulk:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->save_comments_bulk: %s\n" % e)
[inline-code-end]