## Parametri

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Sì |  |

## Risposta

Restituisce: [`AddSSOUserAPIResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/add_sso_user_api_response.py)

## Esempio

[inline-code-attrs-start title = 'Esempio di add_sso_user'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.add_sso_user_api_response import AddSSOUserAPIResponse
from client.models.create_apisso_user_data import CreateAPISSOUserData
from client.rest import ApiException
from pprint import pprint

# La definizione dell'host è opzionale e il valore predefinito è https://fastcomments.com
# Vedi configuration.py per un elenco di tutti i parametri di configurazione supportati.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# Il client deve configurare i parametri di autenticazione e autorizzazione
# in conformità con la politica di sicurezza del server API.
# Esempi per ogni metodo di auth sono forniti di seguito, usa l'esempio che
# soddisfa il tuo caso d'uso di autenticazione.

# Configura l'autorizzazione tramite API key: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# Decommenta qui sotto per impostare un prefisso (es. Bearer) per l'API key, se necessario
# configuration.api_key_prefix['api_key'] = 'Bearer'

# Entra in un contesto con un'istanza del client API
with client.ApiClient(configuration) as api_client:
    # Crea un'istanza della classe API
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    create_apisso_user_data = client.CreateAPISSOUserData() # CreateAPISSOUserData | 

    try:
        api_response = api_instance.add_sso_user(tenant_id, create_apisso_user_data)
        print("The response of DefaultApi->add_sso_user:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->add_sso_user: %s\n" % e)
[inline-code-end]