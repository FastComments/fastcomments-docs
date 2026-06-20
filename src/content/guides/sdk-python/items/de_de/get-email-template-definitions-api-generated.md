## Parameter

| Name | Typ | Location | Erforderlich | Beschreibung |
|------|------|----------|-------------|-------------|
| tenantId | string | query | Ja |  |

## Antwort

Gibt zurück: [`GetEmailTemplateDefinitionsResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/get_email_template_definitions_response.py)

## Beispiel

[inline-code-attrs-start title = 'get_email_template_definitions Beispiel'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.get_email_template_definitions_response import GetEmailTemplateDefinitionsResponse
from client.rest import ApiException
from pprint import pprint

# Das Festlegen des Hosts ist optional und standardmäßig https://fastcomments.com
# Siehe configuration.py für eine Liste aller unterstützten Konfigurationsparameter.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# Der Client muss die Authentifizierungs- und Autorisierungsparameter
# gemäß der Sicherheitsrichtlinie des API-Servers konfigurieren.
# Beispiele für jede Authentifizierungsmethode sind unten aufgeführt, verwenden Sie
# das Beispiel, das Ihrem Auth-Anwendungsfall entspricht.

# API-Schlüssel-Authentifizierung konfigurieren: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# Entfernen Sie die folgende Kommentar-Markierung, um bei Bedarf ein Präfix (z. B. Bearer) für den API-Schlüssel einzurichten
# configuration.api_key_prefix['api_key'] = 'Bearer'

# Einen Kontext mit einer Instanz des API-Clients betreten
with client.ApiClient(configuration) as api_client:
    # Eine Instanz der API-Klasse erstellen
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 

    try:
        api_response = api_instance.get_email_template_definitions(tenant_id)
        print("The response of DefaultApi->get_email_template_definitions:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->get_email_template_definitions: %s\n" % e)
[inline-code-end]