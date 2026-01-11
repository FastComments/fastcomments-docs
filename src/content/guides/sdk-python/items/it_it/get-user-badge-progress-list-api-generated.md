## Parametri

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Sì |  |
| userId | string | query | No |  |
| limit | number | query | No |  |
| skip | number | query | No |  |

## Risposta

Restituisce: [`GetUserBadgeProgressList200Response`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/get_user_badge_progress_list200_response.py)

## Esempio

[inline-code-attrs-start title = 'Esempio di get_user_badge_progress_list'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.get_user_badge_progress_list200_response import GetUserBadgeProgressList200Response
from client.rest import ApiException
from pprint import pprint

# Definire l'host è opzionale e di default è https://fastcomments.com
# Vedi configuration.py per un elenco di tutti i parametri di configurazione supportati.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# Il client deve configurare i parametri di autenticazione e autorizzazione
# in conformità con la policy di sicurezza del server API.
# Degli esempi per ogni metodo di autenticazione sono forniti qui sotto, usa
# l'esempio che soddisfa il tuo caso d'uso di autenticazione.

# Configura l'autorizzazione tramite API key: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# Decommenta quanto segue per impostare un prefisso (es. Bearer) per la API key, se necessario
# configuration.api_key_prefix['api_key'] = 'Bearer'

# Entra in un contesto con un'istanza del client API
with client.ApiClient(configuration) as api_client:
    # Crea un'istanza della classe API
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    user_id = 'user_id_example' # str |  (opzionale)
    limit = 3.4 # float |  (opzionale)
    skip = 3.4 # float |  (opzionale)

    try:
        api_response = api_instance.get_user_badge_progress_list(tenant_id, user_id=user_id, limit=limit, skip=skip)
        print("The response of DefaultApi->get_user_badge_progress_list:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->get_user_badge_progress_list: %s\n" % e)
[inline-code-end]