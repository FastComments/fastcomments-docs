## Parâmetros

| Nome | Tipo | Local | Obrigatório | Descrição |
|------|------|----------|----------|-------------|
| tenantId | string | query | Sim |  |
| page | integer | query | Não |  |
| limit | integer | query | Não |  |
| skip | integer | query | Não |  |
| asTree | boolean | query | Não |  |
| skipChildren | integer | query | Não |  |
| limitChildren | integer | query | Não |  |
| maxTreeDepth | integer | query | Não |  |
| urlId | string | query | Não |  |
| userId | string | query | Não |  |
| anonUserId | string | query | Não |  |
| contextUserId | string | query | Não |  |
| hashTag | string | query | Não |  |
| parentId | string | query | Não |  |
| direction | string | query | Não |  |

## Response

Retorna: [`GetComments200Response`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/get_comments200_response.py)

## Exemplo

[inline-code-attrs-start title = 'Exemplo de get_comments'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.get_comments200_response import GetComments200Response
from client.models.sort_directions import SortDirections
from client.rest import ApiException
from pprint import pprint

# Definir o host é opcional e por padrão é https://fastcomments.com
# Veja configuration.py para uma lista de todos os parâmetros de configuração suportados.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# O cliente deve configurar os parâmetros de autenticação e autorização
# de acordo com a política de segurança do servidor da API.
# Exemplos para cada método de autenticação são fornecidos abaixo, use o exemplo que
# satisfaça seu caso de uso de autenticação.

# Configurar autorização por chave de API: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# Uncomment below to setup prefix (e.g. Bearer) for API key, if needed
# configuration.api_key_prefix['api_key'] = 'Bearer'

# Entre em um contexto com uma instância do cliente da API
with client.ApiClient(configuration) as api_client:
    # Crie uma instância da classe API
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    page = 56 # int |  (opcional)
    limit = 56 # int |  (opcional)
    skip = 56 # int |  (opcional)
    as_tree = True # bool |  (opcional)
    skip_children = 56 # int |  (opcional)
    limit_children = 56 # int |  (opcional)
    max_tree_depth = 56 # int |  (opcional)
    url_id = 'url_id_example' # str |  (opcional)
    user_id = 'user_id_example' # str |  (opcional)
    anon_user_id = 'anon_user_id_example' # str |  (opcional)
    context_user_id = 'context_user_id_example' # str |  (opcional)
    hash_tag = 'hash_tag_example' # str |  (opcional)
    parent_id = 'parent_id_example' # str |  (opcional)
    direction = client.SortDirections() # SortDirections |  (opcional)

    try:
        api_response = api_instance.get_comments(tenant_id, page=page, limit=limit, skip=skip, as_tree=as_tree, skip_children=skip_children, limit_children=limit_children, max_tree_depth=max_tree_depth, url_id=url_id, user_id=user_id, anon_user_id=anon_user_id, context_user_id=context_user_id, hash_tag=hash_tag, parent_id=parent_id, direction=direction)
        print("The response of DefaultApi->get_comments:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->get_comments: %s\n" % e)
[inline-code-end]