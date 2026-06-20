## Parametri

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes |  |
| forceRecalculate | boolean | query | No |  |

## Odgovor

Vraća: [`BulkAggregateQuestionResultsResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/bulk_aggregate_question_results_response.py)

## Primjer

[inline-code-attrs-start title = 'Primjer bulk_aggregate_question_results'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.bulk_aggregate_question_results_request import BulkAggregateQuestionResultsRequest
from client.models.bulk_aggregate_question_results_response import BulkAggregateQuestionResultsResponse
from client.rest import ApiException
from pprint import pprint

# Definisanje hosta je opciono i podrazumijevano je https://fastcomments.com
# Pogledajte configuration.py za listu svih podržanih parametara konfiguracije.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# Klijent mora konfigurirati parametre autentifikacije i autorizacije
# u skladu sa sigurnosnom politikom API servera.
# Ispod su primjeri za svaki metod autentifikacije, koristite primjer koji
# odgovara vašem slučaju upotrebe autentifikacije.

# Konfigurišite autorizaciju API ključa: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# Otkomentarišite donji dio za podešavanje prefiksa (npr. Bearer) za API ključ, ako je potrebno
# configuration.api_key_prefix['api_key'] = 'Bearer'

# Uđite u kontekst sa instancom API klijenta
with client.ApiClient(configuration) as api_client:
    # Kreirajte instancu API klase
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    bulk_aggregate_question_results_request = client.BulkAggregateQuestionResultsRequest() # BulkAggregateQuestionResultsRequest | 
    force_recalculate = True # bool |  (opcionalno)

    try:
        api_response = api_instance.bulk_aggregate_question_results(tenant_id, bulk_aggregate_question_results_request, force_recalculate=force_recalculate)
        print("The response of DefaultApi->bulk_aggregate_question_results:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->bulk_aggregate_question_results: %s\n" % e)
[inline-code-end]