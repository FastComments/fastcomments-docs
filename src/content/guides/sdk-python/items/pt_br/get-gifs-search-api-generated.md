## Parâmetros

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Sim |  |
| search | string | query | Sim |  |
| locale | string | query | Não |  |
| rating | string | query | Não |  |
| page | number | query | Não |  |

## Resposta

Retorna: [`GetGifsSearchResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/get_gifs_search_response.py)

## Exemplo

[inline-code-attrs-start title = 'Exemplo de get_gifs_search'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.get_gifs_search_response import GetGifsSearchResponse
from client.rest import ApiException
from pprint import pprint

# Definir o host é opcional e o padrão é https://fastcomments.com
# Veja configuration.py para uma lista de todos os parâmetros de configuração suportados.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# Entre em um contexto com uma instância do cliente da API
with client.ApiClient(configuration) as api_client:
    # Crie uma instância da classe API
    api_instance = client.PublicApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    search = 'search_example' # str | 
    locale = 'locale_example' # str |  (opcional)
    rating = 'rating_example' # str |  (opcional)
    page = 3.4 # float |  (opcional)

    try:
        api_response = api_instance.get_gifs_search(tenant_id, search, locale=locale, rating=rating, page=page)
        print("The response of PublicApi->get_gifs_search:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling PublicApi->get_gifs_search: %s\n" % e)
[inline-code-end]