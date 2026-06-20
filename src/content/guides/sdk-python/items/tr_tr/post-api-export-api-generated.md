## Parametreler

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| text-search | string | query | Hayır |  |
| byIPFromComment | string | query | Hayır |  |
| filters | string | query | Hayır |  |
| searchFilters | string | query | Hayır |  |
| sorts | string | query | Hayır |  |
| sso | string | query | Hayır |  |

## Yanıt

Döndürür: [`ModerationExportResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/moderation_export_response.py)

## Örnek

[inline-code-attrs-start title = 'post_api_export Örneği'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.moderation_export_response import ModerationExportResponse
from client.rest import ApiException
from pprint import pprint

# Host tanımlaması isteğe bağlıdır ve varsayılan olarak https://fastcomments.com kullanılır
# Tüm desteklenen yapılandırma parametrelerinin listesi için configuration.py dosyasına bakın.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# API istemcisinin bir örneğiyle bir bağlam açın
with client.ApiClient(configuration) as api_client:
    # API sınıfının bir örneğini oluşturun
    api_instance = client.ModerationApi(api_client)
    text_search = 'text_search_example' # str | (opsiyonel)
    by_ip_from_comment = 'by_ip_from_comment_example' # str | (opsiyonel)
    filters = 'filters_example' # str | (opsiyonel)
    search_filters = 'search_filters_example' # str | (opsiyonel)
    sorts = 'sorts_example' # str | (opsiyonel)
    sso = 'sso_example' # str | (opsiyonel)

    try:
        api_response = api_instance.post_api_export(text_search=text_search, by_ip_from_comment=by_ip_from_comment, filters=filters, search_filters=search_filters, sorts=sorts, sso=sso)
        print("The response of ModerationApi->post_api_export:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling ModerationApi->post_api_export: %s\n" % e)
[inline-code-end]