## Parameter

| Name | Typ | Ort | Erforderlich | Beschreibung |
|------|------|----------|----------|-------------|
| tenantId | string | query | Ja |  |

## Antwort

Gibt zurück: [`CreateTenantPackage200Response`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/create_tenant_package200_response.py)

## Beispiel

[inline-code-attrs-start title = 'create_tenant_package Beispiel'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.create_tenant_package200_response import CreateTenantPackage200Response
from client.models.create_tenant_package_body import CreateTenantPackageBody
from client.rest import ApiException
from pprint import pprint

# Die Angabe des Hosts ist optional und standardmäßig https://fastcomments.com
# Siehe configuration.py für eine Liste aller unterstützten Konfigurationsparameter.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# Der Client muss die Authentifizierungs- und Autorisierungsparameter
# entsprechend der Sicherheitsrichtlinie des API-Servers konfigurieren.
# Für jede Authentifizierungsmethode sind unten Beispiele angegeben; verwende das Beispiel, das
# deinen Authentifizierungsfall erfüllt.

# API-Key-Authentifizierung konfigurieren: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# Kommentarzeichen unten entfernen, um ein Präfix (z. B. Bearer) für den API-Schlüssel zu setzen, falls erforderlich
# configuration.api_key_prefix['api_key'] = 'Bearer'

# Einen Kontext mit einer Instanz des API-Clients betreten
with client.ApiClient(configuration) as api_client:
    # Eine Instanz der API-Klasse erstellen
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    create_tenant_package_body = client.CreateTenantPackageBody() # CreateTenantPackageBody | 

    try:
        api_response = api_instance.create_tenant_package(tenant_id, create_tenant_package_body)
        print("The response of DefaultApi->create_tenant_package:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->create_tenant_package: %s\n" % e)
[inline-code-end]