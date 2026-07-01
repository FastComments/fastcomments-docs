## Параметри

| Назва | Тип | Розташування | Обов’язковий | Опис |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes |  |
| value | string | query | No |  |
| filters | string | query | No |  |
| searchFilters | string | query | No |  |
| sso | string | query | No |  |

## Відповідь

Повертає: [`ModerationCommentSearchResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/moderation_comment_search_response.py)

## Приклад

[inline-code-attrs-start title = 'Приклад get_search_comments_summary'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.api.moderation_api import GetSearchCommentsSummaryOptions
from client.models.moderation_comment_search_response import ModerationCommentSearchResponse
from client.rest import ApiException
from pprint import pprint

# Визначення хоста є необов’язковим і за замовчуванням https://fastcomments.com
# Дивіться configuration.py для списку всіх підтримуваних параметрів конфігурації.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# Відкрийте контекст з інстансом API-клієнта
with client.ApiClient(configuration) as api_client:
    # Створіть інстанс класу API
    api_instance = client.ModerationApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    value = 'value_example' # str |  (необов’язковий)
    filters = 'filters_example' # str |  (необов’язковий)
    search_filters = 'search_filters_example' # str |  (необов’язковий)
    sso = 'sso_example' # str |  (необов’язковий)

    try:
        api_response = api_instance.get_search_comments_summary(tenant_id, GetSearchCommentsSummaryOptions(value=value, filters=filters, search_filters=search_filters, sso=sso))
        print("Відповідь ModerationApi->get_search_comments_summary:\n")
        pprint(api_response)
    except Exception as e:
        print("Виняток під час виклику ModerationApi->get_search_comments_summary: %s\n" % e)
[inline-code-end]

---