## Parâmetros

| Nome | Tipo | Localização | Obrigatório | Descrição |
|------|------|-------------|-------------|-----------|
| tenantId | string | query | Yes |  |
| limit | number | query | No |  |
| skip | number | query | No |  |
| order | string | query | No |  |
| after | number | query | No |  |
| before | number | query | No |  |

## Resposta

Retorna: [`GetAuditLogsResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/get_audit_logs_response.py)

## Exemplo

[inline-code-attrs-start title = 'Exemplo get_audit_logs'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.api.default_api import GetAuditLogsOptions
from client.models.get_audit_logs_response import GetAuditLogsResponse
from client.models.sortdir import SORTDIR
from client.rest import ApiException
from pprint import pprint

# Definir o host é opcional e padrão para https://fastcomments.com
# Veja configuration.py para uma lista de todos os parâmetros de configuração suportados.
# O cliente deve configurar os parâmetros de autenticação e autorização
# de acordo com a política de segurança do servidor API.
# Exemplos para cada método de autenticação são fornecidos abaixo, use o exemplo que
# atenda ao seu caso de uso de autenticação.

# Configurar autorização de chave de API: api_key
# Descomente abaixo para definir o prefixo (ex.: Bearer) para a chave de API, se necessário
# Entre em um contexto com uma instância do cliente API
# Crie uma instância da classe API
tenant_id = 'tenant_id_example' # str | 
limit = 3.4 # float |  (optional)
skip = 3.4 # float |  (optional)
order = client.SORTDIR() # SORTDIR |  (optional)
after = 3.4 # float |  (optional)
before = 3.4 # float |  (optional)

try:
    api_response = api_instance.get_audit_logs(tenant_id, GetAuditLogsOptions(limit=limit, skip=skip, order=order, after=after, before=before))
    print("The response of DefaultApi->get_audit_logs:\n")
    pprint(api_response)
except Exception as e:
    print("Exception when calling DefaultApi->get_audit_logs: %s\n" % e)
[inline-code-end]

---