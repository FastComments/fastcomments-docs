---
## Parâmetros

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| page | number | query | Não |  |
| count | number | query | Não |  |
| text-search | string | query | Não |  |
| byIPFromComment | string | query | Não |  |
| filters | string | query | Não |  |
| searchFilters | string | query | Não |  |
| sorts | string | query | Não |  |
| demo | boolean | query | Não |  |
| sso | string | query | Não |  |

## Resposta

Retorna: [`ModerationAPIGetCommentsResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/moderation_api_get_comments_response.py)

## Exemplo

[inline-code-attrs-start title = 'Exemplo de get_api_comments'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.moderation_api_get_comments_response import ModerationAPIGetCommentsResponse
from client.rest import ApiException
from pprint import pprint

# Definir o host é opcional e por padrão é https://fastcomments.com
# Veja configuration.py para uma lista de todos os parâmetros de configuração suportados.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# Entre em um contexto com uma instância do cliente da API
with client.ApiClient(configuration) as api_client:
    # Crie uma instância da classe da API
    api_instance = client.ModerationApi(api_client)
    page = 3.4 # float |  (opcional)
    count = 3.4 # float |  (opcional)
    text_search = 'text_search_example' # str |  (opcional)
    by_ip_from_comment = 'by_ip_from_comment_example' # str |  (opcional)
    filters = 'filters_example' # str |  (opcional)
    search_filters = 'search_filters_example' # str |  (opcional)
    sorts = 'sorts_example' # str |  (opcional)
    demo = True # bool |  (opcional)
    sso = 'sso_example' # str |  (opcional)

    try:
        api_response = api_instance.get_api_comments(page=page, count=count, text_search=text_search, by_ip_from_comment=by_ip_from_comment, filters=filters, search_filters=search_filters, sorts=sorts, demo=demo, sso=sso)
        print("The response of ModerationApi->get_api_comments:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling ModerationApi->get_api_comments: %s\n" % e)
[inline-code-end]

---