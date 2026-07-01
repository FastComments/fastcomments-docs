## Parametri

| Nome | Tipo | Posizione | Obbligatorio | Descrizione |
|------|------|-----------|--------------|-------------|
| tenantId | string | query | Sì |  |
| meta | string | query | No |  |
| skip | number | query | No |  |

## Risposta

Restituisce: [`GetTenantsResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/get_tenants_response.py)

## Esempio

[inline-code-attrs-start title = 'Esempio get_tenants'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.api.default_api import GetTenantsOptions
from client.models.get_tenants_response import GetTenantsResponse
from client.rest import ApiException
from pprint import pprint

# Definire l'host è opzionale e il valore predefinito è https://fastcomments.com
# Vedere configuration.py per un elenco di tutti i parametri di configurazione supportati.
# Il client deve configurare i parametri di autenticazione e autorizzazione in conformità con la policy di sicurezza del server API.
# Esempi per ciascun metodo di autenticazione sono forniti di seguito, usa l'esempio che
# soddisfa il tuo caso d'uso di autenticazione.

# Configura l'autorizzazione della chiave API: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# Decommenta quanto segue per impostare il prefisso (ad esempio Bearer) per la chiave API, se necessario
# configuration.api_key_prefix['api_key'] = 'Bearer'

# Entra in un contesto con un'istanza del client API
with client.ApiClient(configuration) as api_client:
    # Crea un'istanza della classe API
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    meta = 'meta_example' # str |  (opzionale)
    skip = 3.4 # float |  (opzionale)

    try:
        api_response = api_instance.get_tenants(tenant_id, GetTenantsOptions(meta=meta, skip=skip))
        print("The response of DefaultApi->get_tenants:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->get_tenants: %s\n" % e)
[inline-code-end]