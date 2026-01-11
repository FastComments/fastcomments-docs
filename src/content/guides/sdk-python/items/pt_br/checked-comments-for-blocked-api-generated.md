## Parâmetros

| Nome | Tipo | Localização | Obrigatório | Descrição |
|------|------|----------|----------|-------------|
| tenantId | string | query | Sim |  |
| commentIds | string | query | Sim | Uma lista separada por vírgulas de IDs de comentários. |
| sso | string | query | Não |  |

## Resposta

Retorna: [`CheckedCommentsForBlocked200Response`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/checked_comments_for_blocked200_response.py)

## Exemplo

[inline-code-attrs-start title = 'Exemplo de checked_comments_for_blocked'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.checked_comments_for_blocked200_response import CheckedCommentsForBlocked200Response
from client.rest import ApiException
from pprint import pprint

# Definir o host é opcional e padrão é https://fastcomments.com
# Veja configuration.py para uma lista de todos os parâmetros de configuração suportados.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# Entre em um contexto com uma instância do cliente da API
with client.ApiClient(configuration) as api_client:
    # Crie uma instância da classe API
    api_instance = client.PublicApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    comment_ids = 'comment_ids_example' # str | Uma lista separada por vírgulas de IDs de comentários.
    sso = 'sso_example' # str |  (opcional)

    try:
        api_response = api_instance.checked_comments_for_blocked(tenant_id, comment_ids, sso=sso)
        print("The response of PublicApi->checked_comments_for_blocked:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling PublicApi->checked_comments_for_blocked: %s\n" % e)
[inline-code-end]