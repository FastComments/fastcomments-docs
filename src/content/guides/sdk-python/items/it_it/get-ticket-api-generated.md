## Parametri

| Nome | Tipo | Posizione | Richiesto | Descrizione |
|------|------|----------|----------|-------------|
| tenantId | string | query | Sì |  |
| id | string | path | Sì |  |
| userId | string | query | No |  |

## Risposta

Restituisce: [`GetTicket200Response`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/get_ticket200_response.py)

## Esempio

[inline-code-attrs-start title = 'Esempio get_ticket'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.get_ticket200_response import GetTicket200Response
from client.rest import ApiException
from pprint import pprint

# La definizione dell'host è opzionale e per impostazione predefinita è https://fastcomments.com
# Vedere configuration.py per un elenco di tutti i parametri di configurazione supportati.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# Il client deve configurare i parametri di autenticazione e autorizzazione
# in conformità con la policy di sicurezza del server API.
# Gli esempi per ogni metodo di autenticazione sono forniti di seguito; usa quello che
# soddisfa il tuo caso d'uso di autenticazione.

# Configurare l'autorizzazione tramite API key: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# Decommentare sotto per impostare un prefisso (es. Bearer) per la API key, se necessario
# configuration.api_key_prefix['api_key'] = 'Bearer'

# Entrare in un contesto con un'istanza del client API
with client.ApiClient(configuration) as api_client:
    # Creare un'istanza della classe API
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    id = 'id_example' # str | 
    user_id = 'user_id_example' # str |  (opzionale)

    try:
        api_response = api_instance.get_ticket(tenant_id, id, user_id=user_id)
        print("The response of DefaultApi->get_ticket:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->get_ticket: %s\n" % e)
[inline-code-end]