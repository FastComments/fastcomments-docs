---
Esegue aggregazioni sui documenti raggruppandoli (se viene fornito groupBy) e applicando più operazioni. Sono supportate diverse operazioni (ad es. sum, countDistinct, avg, ecc.).

## Parametri

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Sì |  |
| parentTenantId | string | query | No |  |
| includeStats | boolean | query | No |  |

## Risposta

Restituisce: [`AggregateResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/aggregate_response.py)

## Esempio

[inline-code-attrs-start title = 'Esempio di aggregate'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.aggregate_response import AggregateResponse
from client.models.aggregation_request import AggregationRequest
from client.rest import ApiException
from pprint import pprint

# La definizione dell'host è opzionale e predefinita a https://fastcomments.com
# Vedi configuration.py per l'elenco di tutti i parametri di configurazione supportati.
# Il client deve configurare i parametri di autenticazione e autorizzazione
# in conformità con la policy di sicurezza del server API.
# Di seguito sono forniti esempi per ogni metodo di autenticazione; usa quello
# che soddisfa il tuo caso d'uso di autenticazione.
# Configure API key authorization: api_key
# Decommenta quanto sotto per impostare un prefisso (ad es. Bearer) per la API key, se necessario
# configuration.api_key_prefix['api_key'] = 'Bearer'

# Entra in un contesto con un'istanza del client API
with client.ApiClient(configuration) as api_client:
    # Crea un'istanza della classe API
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    aggregation_request = client.AggregationRequest() # AggregationRequest | 
    parent_tenant_id = 'parent_tenant_id_example' # str |  (optional)
    include_stats = True # bool |  (optional)

    try:
        api_response = api_instance.aggregate(tenant_id, aggregation_request, parent_tenant_id=parent_tenant_id, include_stats=include_stats)
        print("The response of DefaultApi->aggregate:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->aggregate: %s\n" % e)
[inline-code-end]

---