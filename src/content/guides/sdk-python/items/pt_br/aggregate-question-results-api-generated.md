## Parâmetros

| Nome | Tipo | Localização | Obrigatório | Descrição |
|------|------|----------|----------|-------------|
| tenantId | string | query | Sim |  |
| questionId | string | query | Não |  |
| questionIds | array | query | Não |  |
| urlId | string | query | Não |  |
| timeBucket | string | query | Não |  |
| startDate | string | query | Não |  |
| forceRecalculate | boolean | query | Não |  |

## Resposta

Retorna: [`AggregateQuestionResultsResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/aggregate_question_results_response.py)

## Exemplo

[inline-code-attrs-start title = 'Exemplo de aggregate_question_results'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.aggregate_question_results_response import AggregateQuestionResultsResponse
from client.models.aggregate_time_bucket import AggregateTimeBucket
from client.rest import ApiException
from pprint import pprint

# Defining the host is optional and defaults to https://fastcomments.com
# Veja configuration.py para uma lista de todos os parâmetros de configuração suportados.
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
# Descomente abaixo para configurar prefixo (por exemplo, Bearer) para a chave de API, se necessário
# configuration.api_key_prefix['api_key'] = 'Bearer'

# Enter a context with an instance of the API client
# Entre em um contexto com uma instância do cliente da API
with client.ApiClient(configuration) as api_client:
    # Create an instance of the API class
    # Crie uma instância da classe API
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    question_id = 'question_id_example' # str |  (opcional)
    question_ids = ['question_ids_example'] # List[str] |  (opcional)
    url_id = 'url_id_example' # str |  (opcional)
    time_bucket = client.AggregateTimeBucket() # AggregateTimeBucket |  (opcional)
    start_date = '2013-10-20T19:20:30+01:00' # datetime |  (opcional)
    force_recalculate = True # bool |  (opcional)

    try:
        api_response = api_instance.aggregate_question_results(tenant_id, question_id=question_id, question_ids=question_ids, url_id=url_id, time_bucket=time_bucket, start_date=start_date, force_recalculate=force_recalculate)
        print("The response of DefaultApi->aggregate_question_results:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->aggregate_question_results: %s\n" % e)
[inline-code-end]