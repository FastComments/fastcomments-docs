## Parâmetros

| Nome | Tipo | Localização | Obrigatório | Descrição |
|------|------|------------|-------------|-----------|
| tenantId | string | path | Sim |  |
| urlId | string | query | Sim |  |
| usernameStartsWith | string | query | Não |  |
| mentionGroupIds | array | query | Não |  |
| sso | string | query | Não |  |
| searchSection | string | query | Não |  |

## Resposta

Retorna: [`SearchUsersResult`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/search_users_result.py)

## Exemplo

[inline-code-attrs-start title = 'Exemplo de search_users'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.api.public_api import SearchUsersOptions
from client.models.search_users_result import SearchUsersResult
from client.rest import ApiException
from pprint import pprint

# Definir o host é opcional e padrão é https://fastcomments.com
# Veja configuration.py para uma lista de todos os parâmetros de configuração suportados.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# Enter a context with an instance of the API client
with client.ApiClient(configuration) as api_client:
    # Create an instance of the API class
    api_instance = client.PublicApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    url_id = 'url_id_example' # str | 
    username_starts_with = 'username_starts_with_example' # str |  (opcional)
    mention_group_ids = ['mention_group_ids_example'] # List[str] |  (opcional)
    sso = 'sso_example' # str |  (opcional)
    search_section = 'search_section_example' # str |  (opcional)

    try:
        api_response = api_instance.search_users(tenant_id, url_id, SearchUsersOptions(username_starts_with=username_starts_with, mention_group_ids=mention_group_ids, sso=sso, search_section=search_section))
        print("A resposta de PublicApi->search_users:\n")
        pprint(api_response)
    except Exception as e:
        print("Exceção ao chamar PublicApi->search_users: %s\n" % e)
[inline-code-end]