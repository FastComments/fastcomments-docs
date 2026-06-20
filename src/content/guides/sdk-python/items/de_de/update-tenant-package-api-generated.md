## Parameter

| Name | Typ | Location | Erforderlich | Beschreibung |
|------|------|----------|----------|-------------|
| tenantId | string | query | Ja |  |
| id | string | path | Ja |  |

## Antwort

Gibt zurück: [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/api_empty_response.py)

## Beispiel

[inline-code-attrs-start title = 'update_tenant_package Beispiel'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.api_empty_response import APIEmptyResponse
from client.models.update_tenant_package_body import UpdateTenantPackageBody
from client.rest import ApiException
from pprint import pprint

# Die Angabe des Hosts ist optional und standardmäßig auf https://fastcomments.com gesetzt.
# Siehe configuration.py für eine Liste aller unterstützten Konfigurationsparameter.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# Der Client muss die Authentifizierungs- und Autorisierungsparameter konfigurieren
# gemäß der Sicherheitsrichtlinie des API-Servers.
# Beispiele für jede Auth-Methode sind unten angegeben, verwenden Sie das Beispiel, das
# Ihren Authentifizierungsfall erfüllt.

# API-Schlüssel-Autorisierung konfigurieren: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# Kommentieren Sie die folgende Zeile aus, um ein Präfix (z. B. Bearer) für den API-Schlüssel einzurichten, falls erforderlich
# configuration.api_key_prefix['api_key'] = 'Bearer'

# Öffnen Sie einen Kontext mit einer Instanz des API-Clients
with client.ApiClient(configuration) as api_client:
    # Erstellen Sie eine Instanz der API-Klasse
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    id = 'id_example' # str | 
    update_tenant_package_body = client.UpdateTenantPackageBody() # UpdateTenantPackageBody | 

    try:
        api_response = api_instance.update_tenant_package(tenant_id, id, update_tenant_package_body)
        print("The response of DefaultApi->update_tenant_package:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->update_tenant_package: %s\n" % e)
[inline-code-end]