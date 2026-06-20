## Parâmetros

| Nome | Tipo | Localização | Obrigatório | Descrição |
|------|------|----------|----------|-------------|
| value | string | query | Não |  |
| filters | string | query | Não |  |
| searchFilters | string | query | Não |  |
| sso | string | query | Não |  |

## Resposta

Retorna: [`ModerationCommentSearchResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/moderation_comment_search_response.py)

## Exemplo

[inline-code-attrs-start title = 'get_search_comments_summary Example'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.moderation_comment_search_response import ModerationCommentSearchResponse
from client.rest import ApiException
from pprint import pprint

# Defining the host is optional and defaults to https://fastcomments.com
# See configuration.py for a list of all supported configuration parameters.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# Enter a context with an instance of the API client
with client.ApiClient(configuration) as api_client:
    # Create an instance of the API class
    api_instance = client.ModerationApi(api_client)
    value = 'value_example' # str |  (opcional)
    filters = 'filters_example' # str |  (opcional)
    search_filters = 'search_filters_example' # str |  (opcional)
    sso = 'sso_example' # str |  (opcional)

    try:
        api_response = api_instance.get_search_comments_summary(value=value, filters=filters, search_filters=search_filters, sso=sso)
        print("The response of ModerationApi->get_search_comments_summary:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling ModerationApi->get_search_comments_summary: %s\n" % e)
[inline-code-end]