## Parameter

| Name | Typ | Ort | Erforderlich | Beschreibung |
|------|------|----------|----------|-------------|
| tenantId | string | query | Ja |  |
| id | string | path | Ja |  |
| contextUserId | string | query | Nein |  |
| isLive | boolean | query | Nein |  |

## Antwort

Gibt zurück: [`DeleteCommentResult`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/delete_comment_result.py)

## Beispiel

[inline-code-attrs-start title = 'delete_comment Beispiel'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.delete_comment_result import DeleteCommentResult
from client.rest import ApiException
from pprint import pprint

# Die Angabe des Hosts ist optional und standardmäßig auf https://fastcomments.com gesetzt
# Siehe configuration.py für eine Liste aller unterstützten Konfigurationsparameter.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# Der Client muss die Authentifizierungs- und Autorisierungsparameter
# gemäß der Sicherheitsrichtlinie des API-Servers konfigurieren.
# Beispiele für jede Auth-Methode sind unten angegeben. Verwenden Sie das
# Beispiel, das Ihren Authentifizierungsanforderungen entspricht.

# API-Key-Authentifizierung konfigurieren: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# Unten auskommentieren, um ein Präfix (z. B. Bearer) für den API-Key zu setzen, falls erforderlich
# configuration.api_key_prefix['api_key'] = 'Bearer'

# Kontext mit einer Instanz des API-Clients öffnen
with client.ApiClient(configuration) as api_client:
    # Eine Instanz der API-Klasse erstellen
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    id = 'id_example' # str | 
    context_user_id = 'context_user_id_example' # str |  (optional)
    is_live = True # bool |  (optional)

    try:
        api_response = api_instance.delete_comment(tenant_id, id, context_user_id=context_user_id, is_live=is_live)
        print("The response of DefaultApi->delete_comment:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->delete_comment: %s\n" % e)
[inline-code-end]

---