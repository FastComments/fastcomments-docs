## Parâmetros

| Nome | Tipo | Location | Obrigatório | Descrição |
|------|------|----------|------------|-----------|
| tenantId | string | query | Sim |  |
| commentId | string | query | Não |  |
| externalId | string | query | Não |  |
| eventType | string | query | Não |  |
| type | string | query | Não |  |
| domain | string | query | Não |  |
| attemptCountGT | number | query | Não |  |
| skip | number | query | Não |  |

## Resposta

Retorna: [`GetPendingWebhookEvents200Response`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/get_pending_webhook_events200_response.py)

## Exemplo

[inline-code-attrs-start title = 'Exemplo de get_pending_webhook_events'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.get_pending_webhook_events200_response import GetPendingWebhookEvents200Response
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
# atende ao seu caso de uso de autenticação.

# Configure API key authorization: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# Descomente abaixo para configurar prefixo (ex.: Bearer) para a chave de API, se necessário
# configuration.api_key_prefix['api_key'] = 'Bearer'

# Entre em um contexto com uma instância do cliente da API
with client.ApiClient(configuration) as api_client:
    # Crie uma instância da classe API
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    comment_id = 'comment_id_example' # str |  (opcional)
    external_id = 'external_id_example' # str |  (opcional)
    event_type = 'event_type_example' # str |  (opcional)
    type = 'type_example' # str |  (opcional)
    domain = 'domain_example' # str |  (opcional)
    attempt_count_gt = 3.4 # float |  (opcional)
    skip = 3.4 # float |  (opcional)

    try:
        api_response = api_instance.get_pending_webhook_events(tenant_id, comment_id=comment_id, external_id=external_id, event_type=event_type, type=type, domain=domain, attempt_count_gt=attempt_count_gt, skip=skip)
        print("The response of DefaultApi->get_pending_webhook_events:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->get_pending_webhook_events: %s\n" % e)
[inline-code-end]