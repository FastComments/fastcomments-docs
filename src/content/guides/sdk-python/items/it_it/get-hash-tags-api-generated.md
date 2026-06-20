## Parametri

| Nome | Tipo | Posizione | Richiesto | Descrizione |
|------|------|----------|----------|-------------|
| tenantId | string | query | Sì |  |
| page | number | query | No |  |

## Risposta

Restituisce: [`GetHashTagsResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/get_hash_tags_response.py)

## Esempio

[inline-code-attrs-start title = 'Esempio di get_hash_tags'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.get_hash_tags_response import GetHashTagsResponse
from client.rest import ApiException
from pprint import pprint

# Definire l'host è opzionale e il valore predefinito è https://fastcomments.com
# Vedi configuration.py per l'elenco di tutti i parametri di configurazione supportati.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# Il client deve configurare i parametri di autenticazione e autorizzazione
# in conformità con la politica di sicurezza del server API.
# Di seguito sono forniti esempi per ogni metodo di autenticazione; usa l'esempio che
# soddisfa il tuo caso d'uso di autenticazione.

# Configure API key authorization: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# Decommenta sotto per impostare un prefisso (es. Bearer) per api_key, se necessario
# configuration.api_key_prefix['api_key'] = 'Bearer'

# Entra in un contesto con un'istanza del client API
with client.ApiClient(configuration) as api_client:
    # Crea un'istanza della classe API
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    page = 3.4 # float |  (opzionale)

    try:
        api_response = api_instance.get_hash_tags(tenant_id, page=page)
        print("The response of DefaultApi->get_hash_tags:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->get_hash_tags: %s\n" % e)
[inline-code-end]