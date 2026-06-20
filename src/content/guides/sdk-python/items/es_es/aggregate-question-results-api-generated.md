## Parámetros

| Name | Tipo | Ubicación | Obligatorio | Descripción |
|------|------|----------|----------|-------------|
| tenantId | string | query | Sí |  |
| questionId | string | query | No |  |
| questionIds | array | query | No |  |
| urlId | string | query | No |  |
| timeBucket | string | query | No |  |
| startDate | string | query | No |  |
| forceRecalculate | boolean | query | No |  |

## Respuesta

Devuelve: [`AggregateQuestionResultsResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/aggregate_question_results_response.py)

## Ejemplo

[inline-code-attrs-start title = 'Ejemplo de aggregate_question_results'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.aggregate_question_results_response import AggregateQuestionResultsResponse
from client.models.aggregate_time_bucket import AggregateTimeBucket
from client.rest import ApiException
from pprint import pprint

# Definir el host es opcional y por defecto es https://fastcomments.com
# Consulte configuration.py para una lista de todos los parámetros de configuración compatibles.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# El cliente debe configurar los parámetros de autenticación y autorización
# de acuerdo con la política de seguridad del servidor de la API.
# A continuación se proporcionan ejemplos para cada método de autenticación; use el ejemplo que
# satisfaga su caso de uso de autenticación.

# Configure la autorización mediante clave API: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# Descomente abajo para configurar el prefijo (p. ej. Bearer) para la clave API, si es necesario
# configuration.api_key_prefix['api_key'] = 'Bearer'

# Entre en un contexto con una instancia del cliente de API
with client.ApiClient(configuration) as api_client:
    # Cree una instancia de la clase API
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

---