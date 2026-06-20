---
## Параметри

| Назва | Тип | Розташування | Обов'язковий | Опис |
|------|------|----------|----------|-------------|
| text-search | string | query | Ні |  |
| byIPFromComment | string | query | Ні |  |
| filters | string | query | Ні |  |
| searchFilters | string | query | Ні |  |
| sorts | string | query | Ні |  |
| sso | string | query | Ні |  |

## Відповідь

Повертає: [`ModerationExportResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/moderation_export_response.py)

## Приклад

[inline-code-attrs-start title = 'Приклад post_api_export'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.moderation_export_response import ModerationExportResponse
from client.rest import ApiException
from pprint import pprint

# Визначення хоста необов'язкове й за замовчуванням дорівнює https://fastcomments.com
# Див. configuration.py для списку всіх підтримуваних параметрів конфігурації.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# Використайте контекст з екземпляром клієнта API
with client.ApiClient(configuration) as api_client:
    # Створіть екземпляр класу API
    api_instance = client.ModerationApi(api_client)
    text_search = 'text_search_example' # str |  (необов'язково)
    by_ip_from_comment = 'by_ip_from_comment_example' # str |  (необов'язково)
    filters = 'filters_example' # str |  (необов'язково)
    search_filters = 'search_filters_example' # str |  (необов'язково)
    sorts = 'sorts_example' # str |  (необов'язково)
    sso = 'sso_example' # str |  (необов'язково)

    try:
        api_response = api_instance.post_api_export(text_search=text_search, by_ip_from_comment=by_ip_from_comment, filters=filters, search_filters=search_filters, sorts=sorts, sso=sso)
        print("The response of ModerationApi->post_api_export:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling ModerationApi->post_api_export: %s\n" % e)
[inline-code-end]

---