## Parametry

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| value | string | query | Nie |  |
| filters | string | query | Nie |  |
| searchFilters | string | query | Nie |  |
| sso | string | query | Nie |  |

## Odpowiedź

Zwraca: [`ModerationCommentSearchResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/moderation_comment_search_response.py)

## Przykład

[inline-code-attrs-start title = 'Przykład get_search_comments_summary'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.moderation_comment_search_response import ModerationCommentSearchResponse
from client.rest import ApiException
from pprint import pprint

# Określenie hosta jest opcjonalne i domyślnie ustawione na https://fastcomments.com
# Zobacz configuration.py, aby uzyskać listę wszystkich obsługiwanych parametrów konfiguracji.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# Użyj kontekstu z instancją klienta API
with client.ApiClient(configuration) as api_client:
    # Utwórz instancję klasy API
    api_instance = client.ModerationApi(api_client)
    value = 'value_example' # str |  (opcjonalne)
    filters = 'filters_example' # str |  (opcjonalne)
    search_filters = 'search_filters_example' # str |  (opcjonalne)
    sso = 'sso_example' # str |  (opcjonalne)

    try:
        api_response = api_instance.get_search_comments_summary(value=value, filters=filters, search_filters=search_filters, sso=sso)
        print("The response of ModerationApi->get_search_comments_summary:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling ModerationApi->get_search_comments_summary: %s\n" % e)
[inline-code-end]