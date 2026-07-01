Past commenters on the page who are NOT currently online. Sorted by displayName.  
Use this after exhausting /users/online to render a "Members" section.  
Cursor pagination on commenterName: server walks the partial {tenantId, urlId, commenterName}  
index from afterName forward via $gt, no $skip cost.

## Parameters

| Ad | Tür | Konum | Gerekli | Açıklama |
|------|------|----------|----------|-------------|
| tenantId | string | path | Evet |  |
| urlId | string | query | Evet | Sayfa URL tanımlayıcısı (sunucu tarafında temizlenmiş). |
| afterName | string | query | Hayır | İmleç: önceki yanıttan nextAfterName'i geçin. |
| afterUserId | string | query | Hayır | İmleç bağlama kırıcı: önceki yanıttan nextAfterUserId'i geçin. afterName ayarlandığında gereklidir, böylece isim eşitlikleri girişleri düşürmez. |

## Response

Döndürür: [`PageUsersOfflineResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/page_users_offline_response.py)

## Example

[inline-code-attrs-start title = 'get_offline_users Örneği'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.api.public_api import GetOfflineUsersOptions
from client.models.page_users_offline_response import PageUsersOfflineResponse
from client.rest import ApiException
from pprint import pprint

# Host tanımlaması isteğe bağlıdır ve varsayılan https://fastcomments.com adresini kullanır
# Tüm desteklenen yapılandırma parametrelerinin listesi için configuration.py dosyasına bakın.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# API istemcisi örneğiyle bir bağlam gir
with client.ApiClient(configuration) as api_client:
    # API sınıfının bir örneğini oluştur
    api_instance = client.PublicApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    url_id = 'url_id_example' # str | Sayfa URL tanımlayıcısı (sunucu tarafında temizlenmiş).
    after_name = 'after_name_example' # str | İmleç: önceki yanıttan nextAfterName'i geçin. (opsiyonel)
    after_user_id = 'after_user_id_example' # str | İmleç bağlama kırıcı: önceki yanıttan nextAfterUserId'i geçin. afterName ayarlandığında gereklidir, böylece isim eşitlikleri girişleri düşürmez. (opsiyonel)

    try:
        api_response = api_instance.get_offline_users(tenant_id, url_id, GetOfflineUsersOptions(after_name=after_name, after_user_id=after_user_id))
        print("The response of PublicApi->get_offline_users:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling PublicApi->get_offline_users: %s\n" % e)
[inline-code-end]