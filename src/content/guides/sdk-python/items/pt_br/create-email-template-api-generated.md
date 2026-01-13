## Parâmetros

| Nome | Tipo | Localização | Obrigatório | Descrição |
|------|------|----------|----------|-------------|
| tenantId | string | query | Sim |  |

## Resposta

Retorna: [`CreateEmailTemplate200Response`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/create_email_template200_response.py)

## Exemplo

[inline-code-attrs-start title = 'Exemplo de create_email_template'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.create_email_template200_response import CreateEmailTemplate200Response
from client.models.create_email_template_body import CreateEmailTemplateBody
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

# Configure a autorização por chave de API: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# Descomente abaixo para configurar o prefixo (por exemplo, Bearer) para a chave de API, se necessário
# configuration.api_key_prefix['api_key'] = 'Bearer'

# Entre em um contexto com uma instância do cliente da API
with client.ApiClient(configuration) as api_client:
    # Crie uma instância da classe da API
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    create_email_template_body = client.CreateEmailTemplateBody() # CreateEmailTemplateBody | 

    try:
        api_response = api_instance.create_email_template(tenant_id, create_email_template_body)
        print("The response of DefaultApi->create_email_template:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->create_email_template: %s\n" % e)
[inline-code-end]

---