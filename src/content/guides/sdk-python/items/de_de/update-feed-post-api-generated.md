## Parameter

| Name | Typ | Ort | Erforderlich | Beschreibung |
|------|------|----------|----------|-------------|
| tenantId | string | query | Ja |  |
| id | string | path | Ja |  |

## Antwort

Gibt zurück: [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/api_empty_response.py)

## Beispiel

[inline-code-attrs-start title = 'update_feed_post Beispiel'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.api_empty_response import APIEmptyResponse
from client.models.feed_post import FeedPost
from client.rest import ApiException
from pprint import pprint

# Das Festlegen des Hosts ist optional und standardmäßig auf https://fastcomments.com eingestellt
# Siehe configuration.py für eine Liste aller unterstützten Konfigurationsparameter.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# Der Client muss die Authentifizierungs- und Autorisierungsparameter
# in Übereinstimmung mit der Sicherheitsrichtlinie des API-Servers konfigurieren.
# Beispiele für jede Auth-Methode sind unten angegeben, verwenden Sie das Beispiel, das
# Ihren Auth-Anwendungsfall erfüllt.

# API-Schlüssel-Authentifizierung konfigurieren: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# Entfernen Sie unten das Kommentarzeichen, um ein Präfix (z. B. Bearer) für den API-Schlüssel einzurichten, falls erforderlich
# configuration.api_key_prefix['api_key'] = 'Bearer'

# Öffnen Sie einen Kontext mit einer Instanz des API-Clients
with client.ApiClient(configuration) as api_client:
    # Erstellen Sie eine Instanz der API-Klasse
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    id = 'id_example' # str | 
    feed_post = client.FeedPost() # FeedPost | 

    try:
        api_response = api_instance.update_feed_post(tenant_id, id, feed_post)
        print("The response of DefaultApi->update_feed_post:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->update_feed_post: %s\n" % e)
[inline-code-end]

---