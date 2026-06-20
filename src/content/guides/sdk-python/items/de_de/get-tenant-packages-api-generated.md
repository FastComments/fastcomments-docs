---
## Parameter

| Name | Typ | Ort | Erforderlich | Beschreibung |
|------|------|----------|----------|-------------|
| tenantId | string | query | Ja |  |
| skip | number | query | Nein |  |

## Antwort

Gibt zurück: [`GetTenantPackagesResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/get_tenant_packages_response.py)

## Beispiel

[inline-code-attrs-start title = 'get_tenant_packages Beispiel'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.get_tenant_packages_response import GetTenantPackagesResponse
from client.rest import ApiException
from pprint import pprint

# Die Angabe des Hosts ist optional und standardmäßig auf https://fastcomments.com gesetzt
# Siehe configuration.py für eine Liste aller unterstützten Konfigurationsparameter.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# Der Client muss die Authentifizierungs- und Autorisierungsparameter
# entsprechend der Sicherheitspolicy des API-Servers konfigurieren.
# Beispiele für jede Authentifizierungsmethode sind unten aufgeführt, verwenden Sie das Beispiel, das
# Ihrem Authentifizierungsfall entspricht.

# Konfigurieren Sie die API-Schlüssel-Authentifizierung: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# Entfernen Sie die Kommentarzeichen unten, um bei Bedarf ein Präfix (z. B. Bearer) für den API-Schlüssel zu setzen
# configuration.api_key_prefix['api_key'] = 'Bearer'

# Einen Kontext mit einer Instanz des API-Clients öffnen
with client.ApiClient(configuration) as api_client:
    # Erstellen Sie eine Instanz der API-Klasse
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    skip = 3.4 # float |  (optional)

    try:
        api_response = api_instance.get_tenant_packages(tenant_id, skip=skip)
        print("The response of DefaultApi->get_tenant_packages:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->get_tenant_packages: %s\n" % e)
[inline-code-end]

---