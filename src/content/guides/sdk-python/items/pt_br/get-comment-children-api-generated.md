## Parâmetros

| Nome | Tipo | Location | Obrigatório | Descrição |
|------|------|----------|----------|-------------|
| commentId | string | path | Sim |  |
| sso | string | query | Não |  |

## Resposta

Retorna: [`ModerationAPIChildCommentsResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/moderation_api_child_comments_response.py)

## Exemplo

[inline-code-attrs-start title = 'Exemplo de get_comment_children'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.moderation_api_child_comments_response import ModerationAPIChildCommentsResponse
from client.rest import ApiException
from pprint import pprint

# Definir o host é opcional e o padrão é https://fastcomments.com
# Consulte configuration.py para uma lista de todos os parâmetros de configuração suportados.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# Abra um contexto com uma instância do cliente de API
with client.ApiClient(configuration) as api_client:
    # Crie uma instância da classe API
    api_instance = client.ModerationApi(api_client)
    comment_id = 'comment_id_example' # str | 
    sso = 'sso_example' # str |  (opcional)

    try:
        api_response = api_instance.get_comment_children(comment_id, sso=sso)
        print("The response of ModerationApi->get_comment_children:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling ModerationApi->get_comment_children: %s\n" % e)
[inline-code-end]