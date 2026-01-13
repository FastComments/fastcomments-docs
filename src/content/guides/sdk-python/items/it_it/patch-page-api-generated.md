---
## Parametri

| Nome | Tipo | Posizione | Obbligatorio | Descrizione |
|------|------|----------|----------|-------------|
| tenantId | string | query | Sì |  |
| id | string | path | Sì |  |

## Response

Restituisce: [`PatchPageAPIResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/patch_page_api_response.py)

## Esempio

[inline-code-attrs-start title = 'Esempio patch_page'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.patch_page_api_response import PatchPageAPIResponse
from client.models.update_api_page_data import UpdateAPIPageData
from client.rest import ApiException
from pprint import pprint

# La definizione dell'host è opzionale e predefinita a https://fastcomments.com
# Vedi configuration.py per un elenco di tutti i parametri di configurazione supportati.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# Il client deve configurare i parametri di autenticazione e autorizzazione
# in conformità con la policy di sicurezza del server API.
# Di seguito sono forniti esempi per ogni metodo di autenticazione, usa l'esempio che
# soddisfa il tuo caso d'uso di autenticazione.

# Configura l'autorizzazione tramite API key: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# Decommenta sotto per impostare un prefisso (es. Bearer) per la API key, se necessario
# configuration.api_key_prefix['api_key'] = 'Bearer'

# Entra in un contesto con un'istanza del client API
with client.ApiClient(configuration) as api_client:
    # Crea un'istanza della classe API
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    id = 'id_example' # str | 
    update_api_page_data = client.UpdateAPIPageData() # UpdateAPIPageData | 

    try:
        api_response = api_instance.patch_page(tenant_id, id, update_api_page_data)
        print("The response of DefaultApi->patch_page:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->patch_page: %s\n" % e)
[inline-code-end]

---