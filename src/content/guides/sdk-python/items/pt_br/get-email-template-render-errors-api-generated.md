## Parâmetros

| Nome | Tipo | Localização | Obrigatório | Descrição |
|------|------|----------|----------|-------------|
| tenantId | string | query | Sim |  |
| id | string | path | Sim |  |
| skip | number | query | Não |  |

## Resposta

Retorna: [`GetEmailTemplateRenderErrors200Response`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/get_email_template_render_errors200_response.py)

## Exemplo

[inline-code-attrs-start title = 'Exemplo de get_email_template_render_errors'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.get_email_template_render_errors200_response import GetEmailTemplateRenderErrors200Response
from client.rest import ApiException
from pprint import pprint

# Definir o host é opcional e o padrão é https://fastcomments.com
# Veja configuration.py para uma lista de todos os parâmetros de configuração suportados.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# O cliente deve configurar os parâmetros de autenticação e autorização
# de acordo com a política de segurança do servidor da API.
# Exemplos para cada método de autenticação são fornecidos abaixo; use o exemplo que
# atende ao seu caso de uso de autenticação.

# Configure a autorização por chave de API: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# Descomente abaixo para configurar o prefixo (por exemplo Bearer) para a chave de API, se necessário
# configuration.api_key_prefix['api_key'] = 'Bearer'

# Entre em um contexto com uma instância do cliente da API
with client.ApiClient(configuration) as api_client:
    # Crie uma instância da classe da API
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    id = 'id_example' # str | 
    skip = 3.4 # float |  (optional)

    try:
        api_response = api_instance.get_email_template_render_errors(tenant_id, id, skip=skip)
        print("The response of DefaultApi->get_email_template_render_errors:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->get_email_template_render_errors: %s\n" % e)
[inline-code-end]