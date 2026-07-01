## Parametreler

| Ad | Tür | Konum | Gerekli | Açıklama |
|------|------|----------|----------|-------------|
| tenantId | string | path | Evet |  |
| locale | string | query | Hayır |  |
| rating | string | query | Hayır |  |
| page | number | query | Hayır |  |

## Yanıt

Döndürür: [`GetGifsTrendingResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/get_gifs_trending_response.py)

## Örnek

[inline-code-attrs-start title = 'get_gifs_trending Örneği'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.api.public_api import GetGifsTrendingOptions
from client.models.get_gifs_trending_response import GetGifsTrendingResponse
from client.rest import ApiException
from pprint import pprint

# Sunucu tanımlama isteğe bağlıdır ve varsayılan olarak https://fastcomments.com adresine sahiptir
# Tüm desteklenen yapılandırma parametrelerinin bir listesini görmek için configuration.py dosyasına bakın.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# Enter a context with an instance of the API client
with client.ApiClient(configuration) as api_client:
    # Create an instance of the API class
    api_instance = client.PublicApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    locale = 'locale_example' # str |  (isteğe bağlı)
    rating = 'rating_example' # str |  (isteğe bağlı)
    page = 3.4 # float |  (isteğe bağlı)

    try:
        api_response = api_instance.get_gifs_trending(tenant_id, GetGifsTrendingOptions(locale=locale, rating=rating, page=page))
        print("The response of PublicApi->get_gifs_trending:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling PublicApi->get_gifs_trending: %s\n" % e)
[inline-code-end]