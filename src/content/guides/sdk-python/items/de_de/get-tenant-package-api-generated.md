## Parameter

| Name | Typ | Ort | Erforderlich | Beschreibung |
|------|------|----------|----------|-------------|
| tenantId | string | query | Ja |  |
| id | string | path | Ja |  |

## Antwort

Gibt zurück: [`GetTenantPackageResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/get_tenant_package_response.py)

## Beispiel

[inline-code-attrs-start title = 'Beispiel für get_tenant_package'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.get_tenant_package_response import GetTenantPackageResponse
from client.rest import ApiException
from pprint import pprint

# Das Festlegen des Hosts ist optional und standardmäßig https://fastcomments.com
# Siehe configuration.py für eine Liste aller unterstützten Konfigurationsparameter.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# Der Client muss die Authentifizierungs- und Autorisierungsparameter
# gemäß der Sicherheitsrichtlinie des API-Servers konfigurieren.
# Beispiele für jede Authentifizierungsmethode sind unten angegeben, verwenden Sie das Beispiel, das
# Ihren Authentifizierungs-Anwendungsfall erfüllt.

# API-Schlüssel-Authentifizierung konfigurieren: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# Entkommentieren Sie die folgende Zeile, um ein Präfix (z. B. Bearer) für den API-Schlüssel zu setzen, falls erforderlich
# configuration.api_key_prefix['api_key'] = 'Bearer'

# Öffnen Sie einen Kontext mit einer Instanz des API-Clients
with client.ApiClient(configuration) as api_client:
    # Erstellen Sie eine Instanz der API-Klasse
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    id = 'id_example' # str | 

    try:
        api_response = api_instance.get_tenant_package(tenant_id, id)
        print("The response of DefaultApi->get_tenant_package:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->get_tenant_package: %s\n" % e)
[inline-code-end]