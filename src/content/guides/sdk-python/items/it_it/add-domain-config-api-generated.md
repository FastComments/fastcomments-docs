---
## Parametri

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Sì |  |

## Risposta

Restituisce: [`AddDomainConfigResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/add_domain_config_response.py)

## Esempio

[inline-code-attrs-start title = 'Esempio di add_domain_config'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.add_domain_config_params import AddDomainConfigParams
from client.models.add_domain_config_response import AddDomainConfigResponse
from client.rest import ApiException
from pprint import pprint

# Definire l'host è facoltativo e predefinito a https://fastcomments.com
# Vedi configuration.py per un elenco di tutti i parametri di configurazione supportati.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# Il client deve configurare i parametri di autenticazione e autorizzazione
# in conformità con la politica di sicurezza del server API.
# Esempi per ogni metodo di autenticazione sono forniti qui sotto, usa l'esempio che
# soddisfa il tuo caso d'uso di autenticazione.

# Configura l'autorizzazione con chiave API: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# Rimuovi il commento qui sotto per impostare un prefisso (es. Bearer) per la chiave API, se necessario
# configuration.api_key_prefix['api_key'] = 'Bearer'

# Entra in un contesto con un'istanza del client API
with client.ApiClient(configuration) as api_client:
    # Crea un'istanza della classe API
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    add_domain_config_params = client.AddDomainConfigParams() # AddDomainConfigParams | 

    try:
        api_response = api_instance.add_domain_config(tenant_id, add_domain_config_params)
        print("The response of DefaultApi->add_domain_config:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->add_domain_config: %s\n" % e)
[inline-code-end]

---