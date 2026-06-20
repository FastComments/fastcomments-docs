## Parameter

| Name | Typ | Location | Erforderlich | Beschreibung |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes |  |
| domainToUpdate | string | path | Yes |  |

## Antwort

Gibt zurück: [`PutDomainConfigResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/put_domain_config_response.py)

## Beispiel

[inline-code-attrs-start title = 'Beispiel für put_domain_config'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.put_domain_config_response import PutDomainConfigResponse
from client.models.update_domain_config_params import UpdateDomainConfigParams
from client.rest import ApiException
from pprint import pprint

# Die Angabe des Hosts ist optional und standardmäßig auf https://fastcomments.com gesetzt.
# Siehe configuration.py für eine Liste aller unterstützten Konfigurationsparameter.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# Der Client muss die Authentifizierungs- und Autorisierungsparameter
# entsprechend der Sicherheitspolitik des API-Servers konfigurieren.
# Beispiele für jede Authentifizierungsmethode sind unten aufgeführt, verwenden Sie
# das Beispiel, das Ihrem Authentifizierungsfall entspricht.

# API-Schlüssel-Autorisierung konfigurieren: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# Entfernen Sie das Kommentarzeichen unten, um ein Präfix (z. B. Bearer) für den API-Schlüssel einzurichten, falls erforderlich
# configuration.api_key_prefix['api_key'] = 'Bearer'

# Öffnen Sie einen Kontext mit einer Instanz des API-Clients
with client.ApiClient(configuration) as api_client:
    # Erstellen Sie eine Instanz der API-Klasse
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    domain_to_update = 'domain_to_update_example' # str | 
    update_domain_config_params = client.UpdateDomainConfigParams() # UpdateDomainConfigParams | 

    try:
        api_response = api_instance.put_domain_config(tenant_id, domain_to_update, update_domain_config_params)
        print("The response of DefaultApi->put_domain_config:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->put_domain_config: %s\n" % e)
[inline-code-end]