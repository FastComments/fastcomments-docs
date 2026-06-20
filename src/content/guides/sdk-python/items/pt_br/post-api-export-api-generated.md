## Parâmetros

| Nome | Tipo | Localização | Obrigatório | Descrição |
|------|------|----------|----------|-------------|
| text-search | string | query | Não |  |
| byIPFromComment | string | query | Não |  |
| filters | string | query | Não |  |
| searchFilters | string | query | Não |  |
| sorts | string | query | Não |  |
| sso | string | query | Não |  |

## Resposta

Retorna: [`ModerationExportResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/moderation_export_response.py)

## Exemplo

[inline-code-attrs-start title = 'Exemplo de post_api_export'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.moderation_export_response import ModerationExportResponse
from client.rest import ApiException
from pprint import pprint

# Definir o host é opcional e por padrão é https://fastcomments.com
# Consulte configuration.py para a lista de todos os parâmetros de configuração suportados.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# Entre em um contexto com uma instância do cliente da API
with client.ApiClient(configuration) as api_client:
    # Crie uma instância da classe API
    api_instance = client.ModerationApi(api_client)
    text_search = 'text_search_example' # str |  (opcional)
    by_ip_from_comment = 'by_ip_from_comment_example' # str |  (opcional)
    filters = 'filters_example' # str |  (opcional)
    search_filters = 'search_filters_example' # str |  (opcional)
    sorts = 'sorts_example' # str |  (opcional)
    sso = 'sso_example' # str |  (opcional)

    try:
        api_response = api_instance.post_api_export(text_search=text_search, by_ip_from_comment=by_ip_from_comment, filters=filters, search_filters=search_filters, sorts=sorts, sso=sso)
        print("The response of ModerationApi->post_api_export:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling ModerationApi->post_api_export: %s\n" % e)
[inline-code-end]