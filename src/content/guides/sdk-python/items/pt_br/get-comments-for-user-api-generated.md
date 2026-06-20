## Parâmetros

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| userId | string | query | Não |  |
| direction | string | query | Não |  |
| repliesToUserId | string | query | Não |  |
| page | number | query | Não |  |
| includei10n | boolean | query | Não |  |
| locale | string | query | Não |  |
| isCrawler | boolean | query | Não |  |

## Resposta

Retorna: [`GetCommentsForUserResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/get_comments_for_user_response.py)

## Exemplo

[inline-code-attrs-start title = 'Exemplo de get_comments_for_user'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.get_comments_for_user_response import GetCommentsForUserResponse
from client.models.sort_directions import SortDirections
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
    user_id = 'user_id_example' # str |  (opcional)
    direction = client.SortDirections() # SortDirections |  (opcional)
    replies_to_user_id = 'replies_to_user_id_example' # str |  (opcional)
    page = 3.4 # float |  (opcional)
    includei10n = True # bool |  (opcional)
    locale = 'locale_example' # str |  (opcional)
    is_crawler = True # bool |  (opcional)

    try:
        api_response = api_instance.get_comments_for_user(user_id=user_id, direction=direction, replies_to_user_id=replies_to_user_id, page=page, includei10n=includei10n, locale=locale, is_crawler=is_crawler)
        print("The response of PublicApi->get_comments_for_user:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling PublicApi->get_comments_for_user: %s\n" % e)
[inline-code-end]