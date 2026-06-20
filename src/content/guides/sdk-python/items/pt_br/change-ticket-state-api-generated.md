## Parâmetros

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Sim |  |
| userId | string | query | Sim |  |
| id | string | path | Sim |  |

## Resposta

Retorna: [`ChangeTicketStateResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/change_ticket_state_response.py)

## Exemplo

[inline-code-attrs-start title = 'Exemplo de change_ticket_state'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.change_ticket_state_body import ChangeTicketStateBody
from client.models.change_ticket_state_response import ChangeTicketStateResponse
from client.rest import ApiException
from pprint import pprint

# A definição do host é opcional e o padrão é https://fastcomments.com
# Veja configuration.py para uma lista de todos os parâmetros de configuração suportados.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# O cliente deve configurar os parâmetros de autenticação e autorização
# de acordo com a política de segurança do servidor da API.
# Exemplos para cada método de autenticação são fornecidos abaixo; use o exemplo que
# satisfaça seu caso de uso de autenticação.

# Configure a autorização por chave de API: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# Descomente abaixo para configurar o prefixo (ex.: Bearer) para a chave de API, se necessário
# configuration.api_key_prefix['api_key'] = 'Bearer'

# Entre em um contexto com uma instância do cliente da API
with client.ApiClient(configuration) as api_client:
    # Crie uma instância da classe API
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    user_id = 'user_id_example' # str | 
    id = 'id_example' # str | 
    change_ticket_state_body = client.ChangeTicketStateBody() # ChangeTicketStateBody | 

    try:
        api_response = api_instance.change_ticket_state(tenant_id, user_id, id, change_ticket_state_body)
        print("The response of DefaultApi->change_ticket_state:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->change_ticket_state: %s\n" % e)
[inline-code-end]