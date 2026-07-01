## Parâmetros

| Nome | Tipo | Localização | Obrigatório | Descrição |
|------|------|-------------|-------------|-----------|
| tenantId | string | query | Sim |  |
| urlId | string | query | Não |  |
| userId | string | query | Não |  |
| startDate | string | query | Não |  |
| questionId | string | query | Não |  |
| questionIds | string | query | Não |  |
| skip | number | query | Não |  |

## Resposta

Retorna: [`GetQuestionResultsResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/get_question_results_response.py)

## Exemplo

[inline-code-attrs-start title = 'Exemplo get_question_results'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.api.default_api import GetQuestionResultsOptions
from client.models.get_question_results_response import GetQuestionResultsResponse
from client.rest import ApiException
from pprint import pprint

# Definir o host é opcional e o padrão é https://fastcomments.com
# Veja configuration.py para uma lista de todos os parâmetros de configuração suportados.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# O cliente deve configurar os parâmetros de autenticação e autorização
# de acordo com a política de segurança do servidor API.
# Exemplos para cada método de autenticação são fornecidos abaixo, use o exemplo que
# atende ao seu caso de uso de autenticação.

# Configurar autorização de chave API: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# Descomente abaixo para configurar o prefixo (ex.: Bearer) para a chave API, se necessário
# configuration.api_key_prefix['api_key'] = 'Bearer'

# Entre em um contexto com uma instância do cliente API
with client.ApiClient(configuration) as api_client:
    # Crie uma instância da classe API
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    url_id = 'url_id_example' # str |  (opcional)
    user_id = 'user_id_example' # str |  (opcional)
    start_date = 'start_date_example' # str |  (opcional)
    question_id = 'question_id_example' # str |  (opcional)
    question_ids = 'question_ids_example' # str |  (opcional)
    skip = 3.4 # float |  (opcional)

    try:
        api_response = api_instance.get_question_results(tenant_id, GetQuestionResultsOptions(url_id=url_id, user_id=user_id, start_date=start_date, question_id=question_id, question_ids=question_ids, skip=skip))
        print("A resposta de DefaultApi->get_question_results:\n")
        pprint(api_response)
    except Exception as e:
        print("Exceção ao chamar DefaultApi->get_question_results: %s\n" % e)
[inline-code-end]