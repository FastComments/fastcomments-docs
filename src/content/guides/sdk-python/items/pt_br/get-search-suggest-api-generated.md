## Parâmetros

| Nome | Tipo | Localização | Obrigatório | Descrição |
|------|------|----------|----------|-------------|
| text-search | string | query | Não |  |
| sso | string | query | Não |  |

## Resposta

Retorna: [`ModerationSuggestResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/moderation_suggest_response.py)

## Exemplo

[inline-code-attrs-start title = 'Exemplo de get_search_suggest'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.moderation_suggest_response import ModerationSuggestResponse
from client.rest import ApiException
from pprint import pprint

# Definir o host é opcional e, por padrão, é https://fastcomments.com
# Veja configuration.py para uma lista de todos os parâmetros de configuração suportados.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# Entre em um contexto com uma instância do ApiClient
with client.ApiClient(configuration) as api_client:
    # Crie uma instância da classe API
    api_instance = client.ModerationApi(api_client)
    text_search = 'text_search_example' # str |  (opcional)
    sso = 'sso_example' # str |  (opcional)

    try:
        api_response = api_instance.get_search_suggest(text_search=text_search, sso=sso)
        print("The response of ModerationApi->get_search_suggest:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling ModerationApi->get_search_suggest: %s\n" % e)
[inline-code-end]