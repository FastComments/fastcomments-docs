---
## Parâmetros

| Nome | Tipo | Localização | Obrigatório | Descrição |
|------|------|----------|----------|-------------|
| commentId | string | path | Sim |  |
| sso | string | query | Não |  |

## Resposta

Retorna: [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/api_empty_response.py)

## Exemplo

[inline-code-attrs-start title = 'Exemplo de post_un_flag_comment'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.api_empty_response import APIEmptyResponse
from client.rest import ApiException
from pprint import pprint

# Definir o host é opcional e o padrão é https://fastcomments.com
# Consulte configuration.py para uma lista de todos os parâmetros de configuração suportados.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# Entre em um contexto com uma instância do cliente da API
with client.ApiClient(configuration) as api_client:
    # Crie uma instância da classe API
    api_instance = client.ModerationApi(api_client)
    comment_id = 'comment_id_example' # str | 
    sso = 'sso_example' # str |  (opcional)

    try:
        api_response = api_instance.post_un_flag_comment(comment_id, sso=sso)
        print("The response of ModerationApi->post_un_flag_comment:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling ModerationApi->post_un_flag_comment: %s\n" % e)
[inline-code-end]

---