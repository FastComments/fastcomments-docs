## Parâmetros

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Sim |  |
| id | string | path | Sim |  |
| skip | number | query | Não |  |

## Resposta

Retorna: [`GetEmailTemplateRenderErrorsResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/get_email_template_render_errors_response.py)

## Exemplo

[inline-code-attrs-start title = 'Exemplo de get_email_template_render_errors'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.get_email_template_render_errors_response import GetEmailTemplateRenderErrorsResponse
from client.rest import ApiException
from pprint import pprint

# Definir o host é opcional e assume https://fastcomments.com por padrão
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

# Uncomment below to setup prefix (e.g. Bearer) for API key, if needed
# configuration.api_key_prefix['api_key'] = 'Bearer'

# Enter a context with an instance of the API client
with client.ApiClient(configuration) as api_client:
    # Create an instance of the API class
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