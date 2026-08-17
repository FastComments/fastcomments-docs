## Параметри

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes |  |
| text-search | string | query | No |  |
| byIPFromComment | string | query | No |  |
| filters | string | query | No |  |
| searchFilters | string | query | No |  |
| afterId | string | query | No |  |
| demo | boolean | query | No |  |
| sso | string | query | No |  |

## Одговор

Враћа: [`ModerationAPIGetCommentIdsResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/moderation_api_get_comment_ids_response.py)

## Пример

[inline-code-attrs-start title = 'get_api_ids Пример'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.api.moderation_api import GetApiIdsOptions
from client.models.moderation_api_get_comment_ids_response import ModerationAPIGetCommentIdsResponse
from client.rest import ApiException
from pprint import pprint

# Дефинисање хоста је опционо и подразумева https://fastcomments.com
# Погледајте configuration.py за листу свих подржаних параметара конфигурације.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# Уђите у контекст са инстанцом API клијента
with client.ApiClient(configuration) as api_client:
    # Креирајте инстанцу API класе
    api_instance = client.ModerationApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    text_search = 'text_search_example' # str |  (опционално)
    by_ip_from_comment = 'by_ip_from_comment_example' # str |  (опционално)
    filters = 'filters_example' # str |  (опционално)
    search_filters = 'search_filters_example' # str |  (опционално)
    after_id = 'after_id_example' # str |  (опционално)
    demo = True # bool |  (опционално)
    sso = 'sso_example' # str |  (опционално)

    try:
        api_response = api_instance.get_api_ids(tenant_id, GetApiIdsOptions(text_search=text_search, by_ip_from_comment=by_ip_from_comment, filters=filters, search_filters=search_filters, after_id=after_id, demo=demo, sso=sso))
        print("The response of ModerationApi->get_api_ids:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling ModerationApi->get_api_ids: %s\n" % e)
[inline-code-end]