## Parâmetros

| Nome | Tipo | Localização | Obrigatório | Descrição |
|------|------|----------|----------|-------------|
| tenantId | string | query | Sim |  |
| userId | string | path | Sim |  |

## Resposta

Retorna: [`APIGetUserBadgeProgressResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/api_get_user_badge_progress_response.py)

## Exemplo

[inline-code-attrs-start title = 'Exemplo de get_user_badge_progress_by_user_id'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.api_get_user_badge_progress_response import APIGetUserBadgeProgressResponse
from client.rest import ApiException
from pprint import pprint

# Definir o host é opcional e o padrão é https://fastcomments.com
# Veja configuration.py para uma lista de todos os parâmetros de configuração suportados.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# O cliente deve configurar os parâmetros de autenticação e autorização
# de acordo com a política de segurança do servidor da API.
# Exemplos para cada método de autenticação são fornecidos abaixo, use o exemplo que
# satisfaça seu caso de uso de autenticação.

# Configure API key authorization: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# Descomente abaixo para configurar prefixo (e.g. Bearer) para API key, se necessário
# configuration.api_key_prefix['api_key'] = 'Bearer'

# Entre em um contexto com uma instância do cliente da API
with client.ApiClient(configuration) as api_client:
    # Crie uma instância da classe API
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    user_id = 'user_id_example' # str | 

    try:
        api_response = api_instance.get_user_badge_progress_by_user_id(tenant_id, user_id)
        print("The response of DefaultApi->get_user_badge_progress_by_user_id:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->get_user_badge_progress_by_user_id: %s\n" % e)
[inline-code-end]