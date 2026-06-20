## Parametreler

| Ad | Tür | Konum | Gerekli | Açıklama |
|------|------|----------|----------|-------------|
| userId | string | query | Hayır |  |
| direction | string | query | Hayır |  |
| repliesToUserId | string | query | Hayır |  |
| page | number | query | Hayır |  |
| includei10n | boolean | query | Hayır |  |
| locale | string | query | Hayır |  |
| isCrawler | boolean | query | Hayır |  |

## Yanıt

Döndürür: [`GetCommentsForUserResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/get_comments_for_user_response.py)

## Örnek

[inline-code-attrs-start title = 'get_comments_for_user Örneği'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.get_comments_for_user_response import GetCommentsForUserResponse
from client.models.sort_directions import SortDirections
from client.rest import ApiException
from pprint import pprint

# Sunucu tanımlamak isteğe bağlıdır ve varsayılan olarak https://fastcomments.com kullanılır
# Tüm desteklenen yapılandırma parametrelerinin listesi için configuration.py dosyasına bakın.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# API istemcisinin bir örneğiyle bir bağlam açın
with client.ApiClient(configuration) as api_client:
    # API sınıfından bir örnek oluşturun
    api_instance = client.PublicApi(api_client)
    user_id = 'user_id_example' # str |  (isteğe bağlı)
    direction = client.SortDirections() # SortDirections |  (isteğe bağlı)
    replies_to_user_id = 'replies_to_user_id_example' # str |  (isteğe bağlı)
    page = 3.4 # float |  (isteğe bağlı)
    includei10n = True # bool |  (isteğe bağlı)
    locale = 'locale_example' # str |  (isteğe bağlı)
    is_crawler = True # bool |  (isteğe bağlı)

    try:
        api_response = api_instance.get_comments_for_user(user_id=user_id, direction=direction, replies_to_user_id=replies_to_user_id, page=page, includei10n=includei10n, locale=locale, is_crawler=is_crawler)
        print("The response of PublicApi->get_comments_for_user:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling PublicApi->get_comments_for_user: %s\n" % e)
[inline-code-end]