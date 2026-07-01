## Parametreler

| Ad | Tür | Konum | Gerekli | Açıklama |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes |  |
| yearNumber | number | query | No |  |
| monthNumber | number | query | No |  |
| dayNumber | number | query | No |  |
| skip | number | query | No |  |

## Yanıt

Döndürür: [`GetTenantDailyUsagesResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/get_tenant_daily_usages_response.py)

## Örnek

[inline-code-attrs-start title = 'get_tenant_daily_usages Örneği'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.api.default_api import GetTenantDailyUsagesOptions
from client.models.get_tenant_daily_usages_response import GetTenantDailyUsagesResponse
from client.rest import ApiException
from pprint import pprint

# Host tanımlamak isteğe bağlıdır ve varsayılan olarak https://fastcomments.com adresini kullanır
# configuration.py dosyasında desteklenen tüm yapılandırma parametrelerinin listesini görebilirsiniz.
# İstemcinin, API sunucusunun güvenlik politikasına göre kimlik doğrulama ve yetkilendirme parametrelerini yapılandırması gerekir.
# API sunucusunun güvenlik politikasına uygun şekilde kimlik doğrulama ve yetkilendirme parametrelerini yapılandırmalıdır.
# Her kimlik doğrulama yöntemi için örnekler aşağıda sağlanmıştır,
# kimlik doğrulama kullanım durumunuza uyan örneği kullanın.

# API anahtarı yetkilendirmesini yapılandır: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# Gerekirse API anahtarı için önek (ör. Bearer) ayarlamak için aşağıdakini yorum dışı bırakın
# configuration.api_key_prefix['api_key'] = 'Bearer'

# API istemcisinin bir örneği ile bir bağlam girin
with client.ApiClient(configuration) as api_client:
    # API sınıfının bir örneğini oluştur
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    year_number = 3.4 # float |  (isteğe bağlı)
    month_number = 3.4 # float |  (isteğe bağlı)
    day_number = 3.4 # float |  (isteğe bağlı)
    skip = 3.4 # float |  (isteğe bağlı)

    try:
        api_response = api_instance.get_tenant_daily_usages(tenant_id, GetTenantDailyUsagesOptions(year_number=year_number, month_number=month_number, day_number=day_number, skip=skip))
        print("DefaultApi->get_tenant_daily_usages çağrısının yanıtı:\n")
        pprint(api_response)
    except Exception as e:
        print("DefaultApi->get_tenant_daily_usages çağrısı sırasında istisna oluştu: %s\n" % e)
[inline-code-end]