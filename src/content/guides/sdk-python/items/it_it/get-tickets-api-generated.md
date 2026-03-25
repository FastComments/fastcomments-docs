## Parametri

| Nome | Tipo | Posizione | Obbligatorio | Descrizione |
|------|------|----------|----------|-------------|
| tenantId | string | query | Sì |  |
| userId | string | query | No |  |
| state | number | query | No |  |
| skip | number | query | No |  |
| limit | number | query | No |  |

## Risposta

Restituisce: [`GetTickets200Response`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/get_tickets200_response.py)

## Esempio

[inline-code-attrs-start title = 'Esempio get_tickets'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.get_tickets200_response import GetTickets200Response
from client.rest import ApiException
from pprint import pprint

# Definire l'host è opzionale e di default è https://fastcomments.com
# Consultare configuration.py per un elenco di tutti i parametri di configurazione supportati.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# Il client deve configurare i parametri di autenticazione e autorizzazione
# in conformità con la politica di sicurezza del server API.
# Di seguito sono forniti esempi per ciascun metodo di autenticazione, usare l'esempio che
# soddisfa il caso d'uso di autenticazione.

# Configurare l'autorizzazione con API key: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# Decommentare la riga sottostante per impostare il prefisso (es. Bearer) per la API key, se necessario
# configuration.api_key_prefix['api_key'] = 'Bearer'

# Entrare in un contesto con un'istanza del client API
with client.ApiClient(configuration) as api_client:
    # Creare un'istanza della classe API
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    user_id = 'user_id_example' # str |  (opzionale)
    state = 3.4 # float |  (opzionale)
    skip = 3.4 # float |  (opzionale)
    limit = 3.4 # float |  (opzionale)

    try:
        api_response = api_instance.get_tickets(tenant_id, user_id=user_id, state=state, skip=skip, limit=limit)
        print("The response of DefaultApi->get_tickets:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->get_tickets: %s\n" % e)
[inline-code-end]