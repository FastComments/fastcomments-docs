## Parâmetros

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Sim |  |
| urlId | string | query | Não |  |
| userId | string | query | Não |  |
| startDate | string | query | Não |  |
| questionId | string | query | Não |  |
| questionIds | string | query | Não |  |
| skip | number | query | Não |  |

## Resposta

Retorna: [`GetQuestionResults200Response`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/get_question_results200_response.py)

## Exemplo

[inline-code-attrs-start title = 'Exemplo de get_question_results'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.get_question_results200_response import GetQuestionResults200Response
from client.rest import ApiException
from pprint import pprint

# Defining the host is optional and defaults to https://fastcomments.com
# Consulte configuration.py para a lista de todos os parâmetros de configuração suportados.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# The client must configure the authentication and authorization parameters
# de acordo com a política de segurança do servidor da API.
# Exemplos para cada método de autenticação são fornecidos abaixo, use o exemplo que
# satisfaça seu caso de uso de autenticação.

# Configure API key authorization: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# Uncomment below to setup prefix (e.g. Bearer) for API key, if needed
# Descomente abaixo para configurar um prefixo (ex.: Bearer) para a chave da API, se necessário

# Enter a context with an instance of the API client
with client.ApiClient(configuration) as api_client:
    # Create an instance of the API class
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    url_id = 'url_id_example' # str |  (opcional)
    user_id = 'user_id_example' # str |  (opcional)
    start_date = 'start_date_example' # str |  (opcional)
    question_id = 'question_id_example' # str |  (opcional)
    question_ids = 'question_ids_example' # str |  (opcional)
    skip = 3.4 # float |  (opcional)

    try:
        api_response = api_instance.get_question_results(tenant_id, url_id=url_id, user_id=user_id, start_date=start_date, question_id=question_id, question_ids=question_ids, skip=skip)
        print("The response of DefaultApi->get_question_results:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->get_question_results: %s\n" % e)
[inline-code-end]

---