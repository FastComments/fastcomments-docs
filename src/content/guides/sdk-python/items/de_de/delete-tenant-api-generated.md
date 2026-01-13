## Parameter

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Ja |  |
| id | string | path | Ja |  |
| sure | string | query | Nein |  |

## Antwort

Gibt zurück: [`FlagCommentPublic200Response`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/flag_comment_public200_response.py)

## Beispiel

[inline-code-attrs-start title = 'delete_tenant Beispiel'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.flag_comment_public200_response import FlagCommentPublic200Response
from client.rest import ApiException
from pprint import pprint

# Die Angabe des Hosts ist optional und standardmäßig https://fastcomments.com
# Siehe configuration.py für eine Liste aller unterstützten Konfigurationsparameter.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# Der Client muss die Authentifizierungs- und Autorisierungsparameter
# entsprechend der Sicherheitsrichtlinie des API-Servers konfigurieren.
# Beispiele für jede Authentifizierungsmethode sind unten angegeben. Verwenden Sie das Beispiel, das
# Ihren Authentifizierungsfall erfüllt.

# API-Key-Autorisierung konfigurieren: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# Unten auskommentierten Code entfernen, um ein Präfix (z. B. Bearer) für den API-Key einzurichten, falls benötigt
# configuration.api_key_prefix['api_key'] = 'Bearer'

# Einen Kontext mit einer Instanz des API-Clients öffnen
with client.ApiClient(configuration) as api_client:
    # Erstelle eine Instanz der API-Klasse
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    id = 'id_example' # str | 
    sure = 'sure_example' # str |  (optional)

    try:
        api_response = api_instance.delete_tenant(tenant_id, id, sure=sure)
        print("The response of DefaultApi->delete_tenant:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->delete_tenant: %s\n" % e)
[inline-code-end]

---