---
## Parameter

| Name | Typ | Location | Erforderlich | Beschreibung |
|------|------|----------|----------|-------------|
| tenantId | string | query | Ja |  |
| id | string | path | Ja |  |

## Antwort

Gibt zurück: [`GetUser200Response`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/get_user200_response.py)

## Beispiel

[inline-code-attrs-start title = 'get_user Beispiel'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.get_user200_response import GetUser200Response
from client.rest import ApiException
from pprint import pprint

# Das Festlegen des Hosts ist optional und standardmäßig auf https://fastcomments.com gesetzt
# Siehe configuration.py für eine Liste aller unterstützten Konfigurationsparameter.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# Der Client muss die Authentifizierungs- und Autorisierungsparameter
# entsprechend der Sicherheitsrichtlinie des API-Servers konfigurieren.
# Beispiele für jede Authentifizierungsmethode sind unten angegeben, verwenden
# Sie das Beispiel, das Ihrem Authentifizierungsfall entspricht.

# Configure API key authorization: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# Entkommentieren Sie unten, um ein Präfix (z. B. Bearer) für den API-Schlüssel einzurichten, falls benötigt
# configuration.api_key_prefix['api_key'] = 'Bearer'

# Betreten Sie einen Kontext mit einer Instanz des API-Clients
with client.ApiClient(configuration) as api_client:
    # Erstellen Sie eine Instanz der API-Klasse
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    id = 'id_example' # str | 

    try:
        api_response = api_instance.get_user(tenant_id, id)
        print("The response of DefaultApi->get_user:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->get_user: %s\n" % e)
[inline-code-end]

---