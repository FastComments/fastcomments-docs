## Parameter

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Ja |  |
| domain | string | path | Ja |  |

## Antwort

Gibt zurück: [`DeleteDomainConfig200Response`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/delete_domain_config200_response.py)

## Beispiel

[inline-code-attrs-start title = 'delete_domain_config Beispiel'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.delete_domain_config200_response import DeleteDomainConfig200Response
from client.rest import ApiException
from pprint import pprint

# Die Definition des Hosts ist optional und standardmäßig auf https://fastcomments.com gesetzt
# Siehe configuration.py für eine Liste aller unterstützten Konfigurationsparameter.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# Der Client muss die Authentifizierungs- und Autorisierungsparameter
# gemäß der Sicherheitsrichtlinie des API-Servers konfigurieren.
# Beispiele für jede Authentifizierungsmethode sind unten angegeben, verwenden Sie das Beispiel, 
# das Ihren Authentifizierungsanforderungen entspricht.

# API-Key-Authentifizierung konfigurieren: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# Auskommentieren, um bei Bedarf ein Präfix (z. B. Bearer) für den API-Key zu setzen
# configuration.api_key_prefix['api_key'] = 'Bearer'

# Betreten eines Kontexts mit einer Instanz des API-Clients
with client.ApiClient(configuration) as api_client:
    # Erstelle eine Instanz der API-Klasse
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    domain = 'domain_example' # str | 

    try:
        api_response = api_instance.delete_domain_config(tenant_id, domain)
        print("The response of DefaultApi->delete_domain_config:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->delete_domain_config: %s\n" % e)
[inline-code-end]