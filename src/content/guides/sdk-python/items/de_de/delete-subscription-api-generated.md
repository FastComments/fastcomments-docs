## Parameter

| Name | Typ | Location | Erforderlich | Beschreibung |
|------|------|----------|----------|-------------|
| tenantId | string | query | Ja |  |
| id | string | path | Ja |  |
| userId | string | query | Nein |  |

## Antwort

Gibt zurück: [`DeleteSubscriptionAPIResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/delete_subscription_api_response.py)

## Beispiel

[inline-code-attrs-start title = 'delete_subscription Beispiel'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.delete_subscription_api_response import DeleteSubscriptionAPIResponse
from client.rest import ApiException
from pprint import pprint

# Die Angabe des Hosts ist optional und standardmäßig https://fastcomments.com
# Siehe configuration.py für eine Liste aller unterstützten Konfigurationsparameter.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# Der Client muss die Authentifizierungs- und Autorisierungsparameter
# entsprechend der Sicherheitsrichtlinie des API-Servers konfigurieren.
# Beispiele für jede Authentifizierungsmethode sind unten aufgeführt, verwenden Sie das Beispiel, das
# Ihren Authentifizierungsfall erfüllt.

# API-Schlüssel-Authentifizierung konfigurieren: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# Entfernen Sie das Kommentarzeichen unten, um bei Bedarf ein Präfix (z. B. Bearer) für den API-Schlüssel zu setzen
# configuration.api_key_prefix['api_key'] = 'Bearer'

# Öffnen Sie einen Kontext mit einer Instanz des API-Clients
with client.ApiClient(configuration) as api_client:
    # Erstellen Sie eine Instanz der API-Klasse
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    id = 'id_example' # str | 
    user_id = 'user_id_example' # str |  (optional)

    try:
        api_response = api_instance.delete_subscription(tenant_id, id, user_id=user_id)
        print("The response of DefaultApi->delete_subscription:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->delete_subscription: %s\n" % e)
[inline-code-end]

---