## Parametreler

| Ad | Tip | Konum | Gerekli | Açıklama |
|------|------|----------|----------|-------------|
| tenantId | string | query | Evet |  |
| sso | string | query | Hayır |  |

## Yanıt

Döndürür: [`APIModerateGetUserBanPreferencesResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/api_moderate_get_user_ban_preferences_response.py)

## Örnek

[inline-code-attrs-start title = 'get_user_ban_preference Örnek'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.api_moderate_get_user_ban_preferences_response import APIModerateGetUserBanPreferencesResponse
from client.rest import ApiException
from pprint import pprint

# Host tanımlamak isteğe bağlıdır ve varsayılan olarak https://fastcomments.com adresine yöneliktir
# Tüm desteklenen yapılandırma parametrelerinin listesini configuration.py dosyasında görebilirsiniz.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# API istemcisinin bir örneğiyle bir bağlam girin
with client.ApiClient(configuration) as api_client:
    # API sınıfının bir örneğini oluştur
    api_instance = client.ModerationApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    sso = 'sso_example' # str |  (isteğe bağlı)

    try:
        api_response = api_instance.get_user_ban_preference(tenant_id, sso=sso)
        print("ModerationApi->get_user_ban_preference yanıtı:\n")
        pprint(api_response)
    except Exception as e:
        print("ModerationApi->get_user_ban_preference çağrılırken oluşan istisna: %s\n" % e)
[inline-code-end]

---