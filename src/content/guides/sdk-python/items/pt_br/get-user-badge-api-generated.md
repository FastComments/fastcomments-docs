## Parâmetros

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Sim |  |
| id | string | path | Sim |  |

## Resposta

Retorna: [`APIGetUserBadgeResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/api_get_user_badge_response.py)

## Exemplo

[inline-code-attrs-start title = 'Exemplo de get_user_badge'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.api_get_user_badge_response import APIGetUserBadgeResponse
from client.rest import ApiException
from pprint import pprint

# Definir o host é opcional e por padrão é https://fastcomments.com
# Veja configuration.py para uma lista de todos os parâmetros de configuração suportados.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# O cliente deve configurar os parâmetros de autenticação e autorização
# de acordo com a política de segurança do servidor da API.
# Exemplos para cada método de autenticação são fornecidos abaixo; use o exemplo que
# atenda ao seu caso de uso de autenticação.

# Configure autorização por chave de API: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# Descomente abaixo para configurar um prefixo (ex.: Bearer) para a chave de API, se necessário
# configuration.api_key_prefix['api_key'] = 'Bearer'

# Entre em um contexto com uma instância do cliente da API
with client.ApiClient(configuration) as api_client:
    # Crie uma instância da classe API
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    id = 'id_example' # str | 

    try:
        api_response = api_instance.get_user_badge(tenant_id, id)
        print("The response of DefaultApi->get_user_badge:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->get_user_badge: %s\n" % e)
[inline-code-end]