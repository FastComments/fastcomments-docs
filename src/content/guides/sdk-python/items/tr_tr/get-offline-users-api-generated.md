Sayfada daha önce yorum yapmış, şu anda çevrimiçi OLMAYAN yorumcular. displayName'e göre sıralanmıştır.
"Üyeler" bölümünü görüntülemek için /users/online tükendikten sonra bunu kullanın.
commenterName üzerinde imleç sayfalama: sunucu, {tenantId, urlId, commenterName} kısmi indeksini
afterName'den $gt ile ileriye doğru tarar, $skip maliyeti yoktur.

## Parametreler

| Ad | Tür | Konum | Zorunlu | Açıklama |
|------|------|----------|----------|-------------|
| tenantId | string | path | Evet |  |
| urlId | string | query | Evet | Sayfa URL tanımlayıcısı (sunucu tarafında temizlenir). |
| afterName | string | query | Hayır | İmleç: önceki yanıttan nextAfterName değerini geçirin. |
| afterUserId | string | query | Hayır | İmleç eşkırıcı: önceki yanıttan nextAfterUserId değerini geçirin. İsim eşitliklerinde girdilerin düşmemesi için afterName ayarlandığında gereklidir. |

## Yanıt

Döndürür: [`PageUsersOfflineResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/page_users_offline_response.py)

## Örnek

[inline-code-attrs-start title = 'get_offline_users Örneği'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.api.public_api import GetOfflineUsersOptions
from client.models.page_users_offline_response import PageUsersOfflineResponse
from client.rest import ApiException
from pprint import pprint

# Host tanımlamak isteğe bağlıdır ve varsayılan olarak https://fastcomments.com değerini alır
# Desteklenen tüm yapılandırma parametrelerinin listesi için configuration.py dosyasına bakın.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# API istemcisinin bir örneğiyle bir bağlam girin
with client.ApiClient(configuration) as api_client:
    # API sınıfının bir örneğini oluşturun
    api_instance = client.PublicApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    url_id = 'url_id_example' # str | Sayfa URL tanımlayıcısı (sunucu tarafında temizlenir).
    after_name = 'after_name_example' # str | İmleç: önceki yanıttan nextAfterName değerini geçirin. (optional)
    after_user_id = 'after_user_id_example' # str | İmleç eşkırıcı: önceki yanıttan nextAfterUserId değerini geçirin. İsim eşitliklerinde girdilerin düşmemesi için afterName ayarlandığında gereklidir. (optional)

    try:
        api_response = api_instance.get_offline_users(tenant_id, url_id, GetOfflineUsersOptions(after_name=after_name, after_user_id=after_user_id))
        print("The response of PublicApi->get_offline_users:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling PublicApi->get_offline_users: %s\n" % e)
[inline-code-end]