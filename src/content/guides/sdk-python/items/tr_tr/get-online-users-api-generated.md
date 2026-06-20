Bir sayfadaki şu anda çevrimiçi izleyiciler: websocket oturumu şu anda sayfaya abone olan kişiler.
anonCount + totalCount döndürür (oda genelindeki aboneler; listelemediğimiz anonim izleyiciler dahil).

## Parametreler

| Ad | Tür | Konum | Gerekli | Açıklama |
|------|------|----------|----------|-------------|
| tenantId | string | path | Yes |  |
| urlId | string | query | Yes | Sayfa URL kimliği (sunucu tarafında temizlenir). |
| afterName | string | query | No | İmleç: önceki yanıttan nextAfterName değerini aktarın. |
| afterUserId | string | query | No | İmleç eşitleyicisi: önceki yanıttan nextAfterUserId değerini aktarın. afterName ayarlandığında, aynı isimli girdilerin düşmemesi için gereklidir. |

## Yanıt

Döndürür: [`PageUsersOnlineResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/page_users_online_response.py)

## Örnek

[inline-code-attrs-start title = 'get_online_users Örneği'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.page_users_online_response import PageUsersOnlineResponse
from client.rest import ApiException
from pprint import pprint

# Host tanımlaması isteğe bağlıdır ve varsayılan olarak https://fastcomments.com kullanılır.
# Desteklenen tüm yapılandırma parametrelerinin listesi için configuration.py dosyasına bakın.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# API istemcisinin bir örneği ile bir bağlam açın
with client.ApiClient(configuration) as api_client:
    # API sınıfının bir örneğini oluşturun
    api_instance = client.PublicApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    url_id = 'url_id_example' # str | Sayfa URL kimliği (sunucu tarafında temizlenir).
    after_name = 'after_name_example' # str | İmleç: önceki yanıttan nextAfterName değerini aktarın. (isteğe bağlı)
    after_user_id = 'after_user_id_example' # str | İmleç eşitleyicisi: önceki yanıttan nextAfterUserId değerini aktarın. afterName ayarlandığında, aynı isimli girdilerin düşmemesi için gereklidir. (isteğe bağlı)

    try:
        api_response = api_instance.get_online_users(tenant_id, url_id, after_name=after_name, after_user_id=after_user_id)
        print("The response of PublicApi->get_online_users:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling PublicApi->get_online_users: %s\n" % e)
[inline-code-end]