## Parametri

| Nome | Tipo | Posizione | Obbligatorio | Descrizione |
|------|------|----------|----------|-------------|
| tenantId | string | query | Sì |  |
| userId | string | query | Sì |  |

## Risposta

Restituisce: [`CreateTicketResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/create_ticket_response.py)

## Esempio

[inline-code-attrs-start title = 'Esempio di create_ticket'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.create_ticket_body import CreateTicketBody
from client.models.create_ticket_response import CreateTicketResponse
from client.rest import ApiException
from pprint import pprint

# Definire l'host è opzionale e per default è https://fastcomments.com
# Vedere configuration.py per la lista di tutti i parametri di configurazione supportati.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# Il client deve configurare i parametri di autenticazione e autorizzazione
# in conformità con la politica di sicurezza del server API.
# Sono forniti esempi per ogni metodo di autenticazione di seguito; usa l'esempio che
# soddisfa il tuo caso d'uso di autenticazione.

# Configure API key authorization: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# Decommentare sotto per impostare il prefisso (e.g. Bearer) per la API key, se necessario
# configuration.api_key_prefix['api_key'] = 'Bearer'

# Entrare in un contesto con un'istanza del client API
with client.ApiClient(configuration) as api_client:
    # Creare un'istanza della classe API
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    user_id = 'user_id_example' # str | 
    create_ticket_body = client.CreateTicketBody() # CreateTicketBody | 

    try:
        api_response = api_instance.create_ticket(tenant_id, user_id, create_ticket_body)
        print("The response of DefaultApi->create_ticket:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->create_ticket: %s\n" % e)
[inline-code-end]