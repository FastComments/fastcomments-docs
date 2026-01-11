## Parametreler

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Evet |  |
| notificationId | string | path | Evet |  |
| newStatus | string | path | Evet |  |
| sso | string | query | Hayır |  |

## Yanıt

Döndürür: [`UpdateUserNotificationStatus200Response`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/update_user_notification_status200_response.py)

## Örnek

[inline-code-attrs-start title = 'update_user_notification_status Örneği'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.update_user_notification_status200_response import UpdateUserNotificationStatus200Response
from client.rest import ApiException
from pprint import pprint

# Sunucu tanımı isteğe bağlıdır ve varsayılan https://fastcomments.com'dur
# Desteklenen tüm yapılandırma parametreleri listesini görmek için configuration.py dosyasına bakın.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# API istemcisinin bir örneği ile bir bağlam açın
with client.ApiClient(configuration) as api_client:
    # API sınıfının bir örneğini oluşturun
    api_instance = client.PublicApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    notification_id = 'notification_id_example' # str | 
    new_status = 'new_status_example' # str | 
    sso = 'sso_example' # str |  (isteğe bağlı)

    try:
        api_response = api_instance.update_user_notification_status(tenant_id, notification_id, new_status, sso=sso)
        print("The response of PublicApi->update_user_notification_status:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling PublicApi->update_user_notification_status: %s\n" % e)
[inline-code-end]

---