## Parameter

| Name | Typ | Ort | Erforderlich | Beschreibung |
|------|------|----------|----------|-------------|
| tenantId | string | query | Ja |  |
| domain | string | path | Ja |  |

## Antwort

Gibt zurück: [`GetDomainConfigResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/get_domain_config_response.py)

## Beispiel

[inline-code-attrs-start title = 'get_domain_config Beispiel'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.get_domain_config_response import GetDomainConfigResponse
from client.rest import ApiException
from pprint import pprint

# Das Festlegen des Hosts ist optional und standardmäßig https://fastcomments.com
# Siehe configuration.py für eine Liste aller unterstützten Konfigurationsparameter.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# Der Client muss die Authentifizierungs- und Autorisierungsparameter
# entsprechend der Sicherheitsrichtlinie des API-Servers konfigurieren.
# Beispiele für jede Authentifizierungsmethode sind unten aufgeführt, verwenden Sie das Beispiel, das
# Ihren Authentifizierungsfall erfüllt.

# API-Schlüsselauthorisierung konfigurieren: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# Entfernen Sie unten das Kommentarzeichen, um ein Präfix (z. B. Bearer) für den API-Schlüssel zu setzen, falls erforderlich
# configuration.api_key_prefix['api_key'] = 'Bearer'

# Öffnen Sie einen Kontext mit einer Instanz des API-Clients
with client.ApiClient(configuration) as api_client:
    # Erstellen Sie eine Instanz der API-Klasse
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    domain = 'domain_example' # str | 

    try:
        api_response = api_instance.get_domain_config(tenant_id, domain)
        print("The response of DefaultApi->get_domain_config:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->get_domain_config: %s\n" % e)
[inline-code-end]