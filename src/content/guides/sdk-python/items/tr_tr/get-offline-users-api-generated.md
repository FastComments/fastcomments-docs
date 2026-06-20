Sayfada daha önce yorum yapmış ve şu anda çevrimiçi olmayan kişiler. displayName ile sıralanır.
/users/online sonrasında "Üyeler" bölümünü göstermek için bunu kullanın.
commenterName üzerinde imleç (cursor) paginasyonu: sunucu kısmi {tenantId, urlId, commenterName}
indeksinde afterName'den itibaren $gt ile ileri doğru ilerler, $skip maliyeti yok.

## Parametreler

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Evet |  |
| urlId | string | query | Evet | Sayfa URL tanımlayıcısı (sunucu tarafında temizlenir). |
| afterName | string | query | Hayır | İmleç: bir önceki yanıttan nextAfterName'i iletin. |
| afterUserId | string | query | Hayır | İmleç çözücü: bir önceki yanıttan nextAfterUserId'i iletin. afterName ayarlandığında aynı isimli kayıtların düşmemesi için gereklidir. |

## Yanıt

Döndürür: [`PageUsersOfflineResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/page_users_offline_response.py)

## Örnek

[inline-code-attrs-start title = 'get_offline_users Example'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.page_users_offline_response import PageUsersOfflineResponse
from client.rest import ApiException
from pprint import pprint

# Host tanımlamak isteğe bağlıdır ve varsayılan olarak https://fastcomments.com kullanılır
# Tüm desteklenen yapılandırma parametrelerinin listesi için configuration.py dosyasına bakın.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# API istemcisinin bir örneği ile bir bağlam içine girin
with client.ApiClient(configuration) as api_client:
    # API sınıfından bir örnek oluşturun
    api_instance = client.PublicApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    url_id = 'url_id_example' # str | Sayfa URL tanımlayıcısı (sunucu tarafında temizlenir).
    after_name = 'after_name_example' # str | İmleç: bir önceki yanıttan nextAfterName'i iletin. (isteğe bağlı)
    after_user_id = 'after_user_id_example' # str | İmleç çözücü: bir önceki yanıttan nextAfterUserId'i iletin. afterName ayarlandığında aynı isimli kayıtların düşmemesi için gereklidir. (isteğe bağlı)

    try:
        api_response = api_instance.get_offline_users(tenant_id, url_id, after_name=after_name, after_user_id=after_user_id)
        print("The response of PublicApi->get_offline_users:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling PublicApi->get_offline_users: %s\n" % e)
[inline-code-end]