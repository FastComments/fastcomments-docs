---
## Parameter

| Name | Typ | Ort | Erforderlich | Beschreibung |
|------|------|----------|----------|-------------|
| tenantId | string | query | Ja |  |
| id | string | path | Ja |  |

## Antwort

Gibt zurück: [`GetTenant200Response`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/get_tenant200_response.py)

## Beispiel

[inline-code-attrs-start title = 'get_tenant Beispiel'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.get_tenant200_response import GetTenant200Response
from client.rest import ApiException
from pprint import pprint

# Die Angabe des Hosts ist optional und standardmäßig https://fastcomments.com
# Siehe configuration.py für eine Liste aller unterstützten Konfigurationsparameter.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# Der Client muss die Authentifizierungs- und Autorisierungsparameter
# gemäß der Sicherheitsrichtlinie des API-Servers konfigurieren.
# Beispiele für jede Authentifizierungsmethode sind unten angegeben; verwenden Sie das Beispiel,
# das Ihren Authentifizierungsfall erfüllt.

# API-Schlüssel-Authentifizierung konfigurieren: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# Um das Präfix (z. B. Bearer) für den API-Schlüssel zu setzen, entfernen Sie bei Bedarf das Kommentarzeichen unten
# configuration.api_key_prefix['api_key'] = 'Bearer'

# Einen Kontext mit einer Instanz des API-Clients betreten
with client.ApiClient(configuration) as api_client:
    # Erstellen Sie eine Instanz der API-Klasse
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    id = 'id_example' # str | 

    try:
        api_response = api_instance.get_tenant(tenant_id, id)
        print("The response of DefaultApi->get_tenant:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->get_tenant: %s\n" % e)
[inline-code-end]

---