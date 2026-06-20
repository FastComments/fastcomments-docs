## Parameter

| Name | Typ | Ort | Erforderlich | Beschreibung |
|------|------|----------|----------|-------------|
| tenantId | string | query | Nein |  |

## Antwort

Gibt zurück: [`CreateHashTagResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/create_hash_tag_response.py)

## Beispiel

[inline-code-attrs-start title = 'add_hash_tag Beispiel'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.create_hash_tag_body import CreateHashTagBody
from client.models.create_hash_tag_response import CreateHashTagResponse
from client.rest import ApiException
from pprint import pprint

# Die Angabe des Hosts ist optional und standardmäßig https://fastcomments.com
# Siehe configuration.py für eine Liste aller unterstützten Konfigurationsparameter.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# Der Client muss die Authentifizierungs- und Autorisierungsparameter
# gemäß der Sicherheitspolicy des API-Servers konfigurieren.
# Für jede Authentifizierungsmethode sind unten Beispiele aufgeführt, verwenden Sie das Beispiel, das
# Ihren Authentifizierungsfall erfüllt.

# API-Key-Authentifizierung konfigurieren: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# Wenn nötig, die folgende Zeile auskommentieren, um ein Präfix (z. B. Bearer) für den API-Schlüssel zu setzen
# configuration.api_key_prefix['api_key'] = 'Bearer'

# Einen Kontext mit einer Instanz des API-Clients verwenden
with client.ApiClient(configuration) as api_client:
    # Erstellen Sie eine Instanz der API-Klasse
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str |  (optional)
    create_hash_tag_body = client.CreateHashTagBody() # CreateHashTagBody |  (optional)

    try:
        api_response = api_instance.add_hash_tag(tenant_id=tenant_id, create_hash_tag_body=create_hash_tag_body)
        print("The response of DefaultApi->add_hash_tag:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->add_hash_tag: %s\n" % e)
[inline-code-end]