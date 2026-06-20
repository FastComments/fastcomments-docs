## Parâmetros

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| commentId | string | path | Sim |  |
| sso | string | query | Não |  |

## Resposta

Retorna: [`SetCommentTextResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/set_comment_text_response.py)

## Exemplo

[inline-code-attrs-start title = 'Exemplo post_set_comment_text'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.set_comment_text_params import SetCommentTextParams
from client.models.set_comment_text_response import SetCommentTextResponse
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
    comment_id = 'comment_id_example' # str | 
    set_comment_text_params = client.SetCommentTextParams() # SetCommentTextParams | 
    sso = 'sso_example' # str |  (optional)

    try:
        api_response = api_instance.post_set_comment_text(comment_id, set_comment_text_params, sso=sso)
        print("The response of ModerationApi->post_set_comment_text:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling ModerationApi->post_set_comment_text: %s\n" % e)
[inline-code-end]