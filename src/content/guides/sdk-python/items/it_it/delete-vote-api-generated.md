## Parametri

| Nome | Tipo | Posizione | Richiesto | Descrizione |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes |  |
| id | string | path | Yes |  |
| editKey | string | query | No |  |

## Risposta

Restituisce: [`VoteDeleteResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/vote_delete_response.py)

## Esempio

[inline-code-attrs-start title = 'Esempio delete_vote'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.vote_delete_response import VoteDeleteResponse
from client.rest import ApiException
from pprint import pprint

# Definire l'host è opzionale e per impostazione predefinita è https://fastcomments.com
# Consulta configuration.py per un elenco di tutti i parametri di configurazione supportati.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# Il client deve configurare i parametri di autenticazione e autorizzazione
# in conformità con la policy di sicurezza del server API.
# Gli esempi per ciascun metodo di autenticazione sono forniti qui sotto; usa l'esempio che
# soddisfa il tuo caso d'uso di autenticazione.

# Configura l'autorizzazione tramite API key: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# Decommenta la riga sottostante per impostare un prefisso (es. Bearer) per la API key, se necessario
# configuration.api_key_prefix['api_key'] = 'Bearer'

# Entra in un contesto con un'istanza del client API
with client.ApiClient(configuration) as api_client:
    # Crea un'istanza della classe API
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    id = 'id_example' # str | 
    edit_key = 'edit_key_example' # str |  (opzionale)

    try:
        api_response = api_instance.delete_vote(tenant_id, id, edit_key=edit_key)
        print("The response of DefaultApi->delete_vote:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->delete_vote: %s\n" % e)
[inline-code-end]