## Parameters

| Nome | Tipo | Localização | Obrigatório | Descrição |
|------|------|-------------|--------------|-----------|
| tenantId | string | path | Sim |  |
| commentId | string | path | Sim |  |
| editKey | string | query | Não |  |
| sso | string | query | Não |  |

## Response

Retorna: [`PublicAPIGetCommentTextResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/public_api_get_comment_text_response.py)

## Example

[inline-code-attrs-start title = 'Exemplo get_comment_text'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.api.public_api import GetCommentTextOptions
from client.models.public_api_get_comment_text_response import PublicAPIGetCommentTextResponse
from client.rest import ApiException
from pprint import pprint

# Definir o host é opcional e padrão é https://fastcomments.com
# Consulte configuration.py para obter a lista de todos os parâmetros de configuração suportados.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# Entrar em um contexto com uma instância do cliente API
with client.ApiClient(configuration) as api_client:
    # Criar uma instância da classe API
    api_instance = client.PublicApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    comment_id = 'comment_id_example' # str | 
    edit_key = 'edit_key_example' # str |  (optional)
    sso = 'sso_example' # str |  (optional)

    try:
        api_response = api_instance.get_comment_text(tenant_id, comment_id, GetCommentTextOptions(edit_key=edit_key, sso=sso))
        print("The response of PublicApi->get_comment_text:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling PublicApi->get_comment_text: %s\n" % e)
[inline-code-end]