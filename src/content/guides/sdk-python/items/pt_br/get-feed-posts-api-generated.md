---
req
tenantId
afterId

## Parâmetros

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Sim |  |
| afterId | string | query | Não |  |
| limit | integer | query | Não |  |
| tags | array | query | Não |  |

## Resposta

Retorna: [`GetFeedPostsResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/get_feed_posts_response.py)

## Exemplo

[inline-code-attrs-start title = 'Exemplo de get_feed_posts'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.get_feed_posts_response import GetFeedPostsResponse
from client.rest import ApiException
from pprint import pprint

# A definição do host é opcional e o padrão é https://fastcomments.com
# Consulte configuration.py para uma lista de todos os parâmetros de configuração suportados.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# O cliente deve configurar os parâmetros de autenticação e autorização
# de acordo com a política de segurança do servidor da API.
# Exemplos para cada método de autenticação são fornecidos abaixo, use o exemplo que
# satisfaça seu caso de uso de autenticação.

# Configure a autorização por chave de API: api_key
# Uncomment below to setup prefix (e.g. Bearer) for API key, if needed
# configuration.api_key_prefix['api_key'] = 'Bearer'

# Entre em um contexto com uma instância do cliente da API
with client.ApiClient(configuration) as api_client:
    # Crie uma instância da classe API
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    after_id = 'after_id_example' # str |  (optional)
    limit = 56 # int |  (optional)
    tags = ['tags_example'] # List[str] |  (optional)

    try:
        api_response = api_instance.get_feed_posts(tenant_id, after_id=after_id, limit=limit, tags=tags)
        print("The response of DefaultApi->get_feed_posts:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->get_feed_posts: %s\n" % e)
[inline-code-end]

---