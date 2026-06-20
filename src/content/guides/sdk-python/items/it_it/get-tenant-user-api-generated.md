## Parametri

| Nome | Tipo | Posizione | Obbligatorio | Descrizione |
|------|------|----------|----------|-------------|
| tenantId | string | query | Sì |  |
| id | string | path | Sì |  |

## Risposta

Restituisce: [`GetTenantUserResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/get_tenant_user_response.py)

## Esempio

[inline-code-attrs-start title = 'Esempio di get_tenant_user'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.get_tenant_user_response import GetTenantUserResponse
from client.rest import ApiException
from pprint import pprint

# Definire l'host è opzionale e per impostazione predefinita è https://fastcomments.com
# Vedi configuration.py per un elenco di tutti i parametri di configurazione supportati.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# Il client deve configurare i parametri di autenticazione e autorizzazione
# in conformità con la policy di sicurezza del server API.
# Di seguito sono forniti esempi per ogni metodo di autenticazione; usa l'esempio che
# soddisfa il tuo caso d'uso di autenticazione.

# Configurare l'autorizzazione tramite API key: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# Decommenta la riga sottostante per impostare un prefisso (es. Bearer) per l'API key, se necessario
# configuration.api_key_prefix['api_key'] = 'Bearer'

# Entra in un contesto con un'istanza del client API
with client.ApiClient(configuration) as api_client:
    # Crea un'istanza della classe API
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    id = 'id_example' # str | 

    try:
        api_response = api_instance.get_tenant_user(tenant_id, id)
        print("The response of DefaultApi->get_tenant_user:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->get_tenant_user: %s\n" % e)
[inline-code-end]