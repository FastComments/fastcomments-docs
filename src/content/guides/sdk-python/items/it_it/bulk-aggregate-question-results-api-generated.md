## Parametri

| Nome | Tipo | Posizione | Obbligatorio | Descrizione |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes |  |
| forceRecalculate | boolean | query | No |  |

## Risposta

Restituisce: [`BulkAggregateQuestionResults200Response`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/bulk_aggregate_question_results200_response.py)

## Esempio

[inline-code-attrs-start title = 'Esempio di bulk_aggregate_question_results'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.bulk_aggregate_question_results200_response import BulkAggregateQuestionResults200Response
from client.models.bulk_aggregate_question_results_request import BulkAggregateQuestionResultsRequest
from client.rest import ApiException
from pprint import pprint

# Definire l'host è opzionale e il valore predefinito è https://fastcomments.com
# Vedi configuration.py per un elenco di tutti i parametri di configurazione supportati.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# Il client deve configurare i parametri di autenticazione e autorizzazione
# in conformità con la politica di sicurezza del server API.
# Sono forniti esempi per ogni metodo di autenticazione qui sotto, usa l'esempio che
# soddisfa il tuo caso d'uso di autenticazione.

# Configura l'autorizzazione tramite API key: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# Decommenta la riga sottostante per impostare il prefisso (es. Bearer) per la API key, se necessario
# configuration.api_key_prefix['api_key'] = 'Bearer'

# Entra in un contesto con un'istanza del client API
with client.ApiClient(configuration) as api_client:
    # Crea un'istanza della classe API
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    bulk_aggregate_question_results_request = client.BulkAggregateQuestionResultsRequest() # BulkAggregateQuestionResultsRequest | 
    force_recalculate = True # bool |  (optional)

    try:
        api_response = api_instance.bulk_aggregate_question_results(tenant_id, bulk_aggregate_question_results_request, force_recalculate=force_recalculate)
        print("The response of DefaultApi->bulk_aggregate_question_results:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->bulk_aggregate_question_results: %s\n" % e)
[inline-code-end]