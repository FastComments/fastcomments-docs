## Parametreler

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Evet |  |
| sso | string | query | Hayır |  |

## Yanıt

Döndürür: [`GetUserNotificationCountResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/get_user_notification_count_response.py)

## Örnek

[inline-code-attrs-start title = 'get_user_notification_count Örneği'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.get_user_notification_count_response import GetUserNotificationCountResponse
from client.rest import ApiException
from pprint import pprint

# Host tanımı isteğe bağlıdır ve varsayılan olarak https://fastcomments.com kullanılır
# Tüm desteklenen yapılandırma parametrelerinin listesi için configuration.py dosyasına bakın.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# API istemcisinin bir örneği ile bir bağlam açın
with client.ApiClient(configuration) as api_client:
    # API sınıfının bir örneğini oluşturun
    api_instance = client.PublicApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    sso = 'sso_example' # str |  (isteğe bağlı)

    try:
        api_response = api_instance.get_user_notification_count(tenant_id, sso=sso)
        print("The response of PublicApi->get_user_notification_count:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling PublicApi->get_user_notification_count: %s\n" % e)
[inline-code-end]