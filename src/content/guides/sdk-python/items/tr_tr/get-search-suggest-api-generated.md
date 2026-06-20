## Parametreler

| Ad | Tür | Konum | Gerekli | Açıklama |
|------|------|----------|----------|-------------|
| text-search | string | query | Hayır |  |
| sso | string | query | Hayır |  |

## Yanıt

Döndürür: [`ModerationSuggestResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/moderation_suggest_response.py)

## Örnek

[inline-code-attrs-start title = 'get_search_suggest Örneği'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.moderation_suggest_response import ModerationSuggestResponse
from client.rest import ApiException
from pprint import pprint

# Host tanımlamak isteğe bağlıdır ve varsayılan olarak https://fastcomments.com kullanılır
# Tüm desteklenen yapılandırma parametreleri listesi için configuration.py dosyasına bakın.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# API istemcisinin bir örneğiyle bir bağlam açın
with client.ApiClient(configuration) as api_client:
    # API sınıfının bir örneğini oluşturun
    api_instance = client.ModerationApi(api_client)
    text_search = 'text_search_example' # str |  (isteğe bağlı)
    sso = 'sso_example' # str |  (isteğe bağlı)

    try:
        api_response = api_instance.get_search_suggest(text_search=text_search, sso=sso)
        print("The response of ModerationApi->get_search_suggest:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling ModerationApi->get_search_suggest: %s\n" % e)
[inline-code-end]